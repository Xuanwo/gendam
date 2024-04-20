mod models;

use crate::library::{
    get_library_settings, set_library_settings, LibrarySettings, LIBRARY_SETTINGS_FILE_NAME,
};
use crate::CtxWithLibrary;
use content_library::{create_library, list_library_dirs};
use rspc::{Router, RouterBuilder};
use serde::Serialize;
use serde_json::json;
use specta::Type;
use std::path::PathBuf;

pub fn get_routes<TCtx>() -> RouterBuilder<TCtx>
where
    TCtx: CtxWithLibrary + Clone + Send + Sync + 'static,
{
    Router::<TCtx>::new()
        .query("list", {
            #[derive(Serialize, Type)]
            #[serde(rename_all = "camelCase")]
            pub struct LibrariesListResult {
                pub id: String,
                pub dir: String,
                pub title: String,
            }
            |t| {
                t(|ctx, _input: ()| async move {
                    let library_dirs = list_library_dirs(&ctx.get_local_data_root());
                    library_dirs
                        .into_iter()
                        .map(|(dir, id)| {
                            let title = match std::fs::File::open(
                                PathBuf::from(&dir).join(LIBRARY_SETTINGS_FILE_NAME),
                            ) {
                                Ok(file) => {
                                    let reader = std::io::BufReader::new(file);
                                    match serde_json::from_reader::<_, serde_json::Value>(reader) {
                                        Ok(values) => values["title"]
                                            .as_str()
                                            .unwrap_or("Untitled")
                                            .to_string(),
                                        Err(_) => "Untitled".to_string(),
                                    }
                                }
                                Err(_) => "Untitled".to_string(),
                            };
                            LibrariesListResult { id, dir, title }
                        })
                        .collect::<Vec<LibrariesListResult>>()
                })
            }
        })
        .mutation("create", |t| {
            t(|ctx, title: String| async move {
                let library_dir = create_library(&ctx.get_local_data_root()).await;
                match std::fs::File::create(library_dir.join(LIBRARY_SETTINGS_FILE_NAME)) {
                    Ok(file) => {
                        let value = serde_json::json!({ "title": title });
                        if let Err(e) = serde_json::to_writer(file, &value) {
                            tracing::error!("Failed to write file: {}", e);
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to create file: {}", e);
                    }
                };
                // json!({ "id": library.id, "dir": library.dir })
            })
        })
        .query("get_library_settings", |t| {
            t(|ctx, _input: ()| async move {
                let library = ctx.library()?;
                let settings = get_library_settings(&library.dir);
                Ok(settings)
            })
        })
        .mutation("update_library_settings", |t| {
            t(|ctx, input: LibrarySettings| async move {
                let library = ctx.library()?;
                set_library_settings(&library.dir, input);
                Ok(())
            })
        })
        .mutation("set_current_library", |t| {
            /*
             * ctx.load_library 会负责进行当前 library 的卸载
             */
            t(|ctx, library_id: String| async move {
                if let Err(e) = ctx.load_library(&library_id).await {
                    tracing::error!("Failed to load library: {}", e);
                }
                json!({ "status": "ok" })
            })
        })
        .mutation("unload_library", |t| {
            t(|ctx, _: Option<serde_json::Value>| async move {
                /*
                 * TODO 如果这里不加一个参数直接用 _input: (), 会因参数校验失败而返回错误,
                 * 因为前端会发一个 payload: `{}`, 而不是空
                 * 所以这里就用 serde_json::Value | None 来允许接收任何值
                 */
                // ctx.library()?;  // 不需要确认 library 存在, 意外情况下可能 library 已经清空但是 task 和 qdrant 还在
                ctx.unload_library()?;
                Ok(json!({ "status": "ok" }))
            })
        })
        .query("get_current_library", |t| {
            t(|ctx, _input: ()| async move {
                #[derive(Serialize, Type)]
                #[serde(rename_all = "camelCase")]
                pub struct CurrentLibraryResult {
                    pub id: String,
                    pub dir: String,
                }
                let library = match ctx.library() {
                    Ok(lib) => lib,
                    // 如果当前 library不存在，需要从本地 store 中将其读取出来
                    _ => {
                        let library_id = ctx.library_id_in_store();
                        // 如果本地 store 中也没有，说明没有设置，返回错误
                        let library_id = library_id.ok_or(rspc::Error::new(
                            rspc::ErrorCode::InternalServerError,
                            "current library not set".into(),
                        ))?;
                        ctx.load_library(&library_id).await?;
                        ctx.library()?
                    }
                };

                Ok(CurrentLibraryResult {
                    id: library.id.clone(),
                    dir: library.dir.to_str().unwrap().to_string(),
                })
            })
        })
        // .query("download_status_by_file_name", |t| {
        //     t(|ctx, file_name: String| async move {
        //         let download_status = ctx.download_status()?;
        //         let download_status = download_status
        //             .iter()
        //             .find(|status| status.file_name == file_name)
        //             .cloned();
        //         Ok(download_status)
        //     })
        // })
        .merge("models.", models::get_routes())
}
