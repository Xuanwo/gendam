mod create;
mod delete;
mod process;
mod read;
mod types;
mod update;
mod utils;

use crate::CtxWithLibrary;
use rspc::{Router, RouterBuilder};
use serde::Deserialize;
use specta::Type;
use tracing::info;

use crate::validators;
use create::{create_asset_object, create_file_path};
use delete::delete_file_path;
use process::{process_video_asset, process_video_metadata};
use read::{get_file_path, list_file_path};
use types::FilePathRequestPayload;
use update::{move_file_path, rename_file_path};

pub fn get_routes<TCtx>() -> RouterBuilder<TCtx>
where
    TCtx: CtxWithLibrary + Clone + Send + Sync + 'static,
{
    Router::<TCtx>::new()
        .mutation("create_file_path", |t| {
            t({
                #[derive(Deserialize, Type, Debug)]
                #[serde(rename_all = "camelCase")]
                struct FilePathCreatePayload {
                    #[serde(deserialize_with = "validators::materialized_path_string")]
                    materialized_path: String,
                    #[serde(deserialize_with = "validators::path_name_string")]
                    name: String,
                }
                |ctx, input: FilePathCreatePayload| async move {
                    let library = ctx.library()?;
                    create_file_path(&library, &input.materialized_path, &input.name).await?;
                    Ok(())
                }
            })
        })
        .mutation("create_asset_object", |t| {
            t({
                #[derive(Deserialize, Type, Debug)]
                #[serde(rename_all = "camelCase")]
                struct AssetObjectCreatePayload {
                    #[serde(deserialize_with = "validators::materialized_path_string")]
                    materialized_path: String,
                    local_full_path: String,
                }
                |ctx: TCtx, input: AssetObjectCreatePayload| async move {
                    info!("received create_asset_object: {input:?}");
                    let library = ctx.library()?;
                    let (file_path_data, asset_object_data, asset_object_existed) =
                        create_asset_object(
                            &library,
                            &input.materialized_path,
                            &input.local_full_path,
                        )
                        .await?;

                    if !asset_object_existed {
                        process_video_metadata(&library, asset_object_data.id).await?;
                        info!("process video metadata finished");
                        process_video_asset(&library, &ctx, file_path_data.id).await?;
                        info!("process video asset finished");
                    }
                    Ok(())
                }
            })
        })
        .query("list", |t| {
            t({
                #[derive(Deserialize, Type, Debug)]
                #[serde(rename_all = "camelCase")]
                struct FilePathQueryPayload {
                    #[serde(deserialize_with = "validators::materialized_path_string")]
                    materialized_path: String,
                    // #[serde(rename = "dirsOnly")]
                    dirs_only: bool,
                }
                |ctx, input: FilePathQueryPayload| async move {
                    let library = ctx.library()?;
                    let names =
                        list_file_path(&library, &input.materialized_path, input.dirs_only).await?;
                    Ok(names)
                }
            })
        })
        .query("get", |t| {
            t({
                #[derive(Deserialize, Type, Debug)]
                #[serde(rename_all = "camelCase")]
                struct FilePathGetPayload {
                    #[serde(deserialize_with = "validators::materialized_path_string")]
                    materialized_path: String,
                    #[serde(deserialize_with = "validators::path_name_string")]
                    name: String,
                }
                |ctx, input: FilePathGetPayload| async move {
                    let library = ctx.library()?;
                    let item =
                        get_file_path(&library, &input.materialized_path, &input.name).await?;
                    Ok(item)
                }
            })
        })
        .mutation("rename_file_path", |t| {
            t({
                #[derive(Deserialize, Type, Debug)]
                #[serde(rename_all = "camelCase")]
                struct FilePathRenamePayload {
                    id: i32,
                    is_dir: bool,
                    #[serde(deserialize_with = "validators::materialized_path_string")]
                    materialized_path: String,
                    #[serde(deserialize_with = "validators::path_name_string")]
                    old_name: String,
                    #[serde(deserialize_with = "validators::path_name_string")]
                    new_name: String,
                }
                |ctx, input: FilePathRenamePayload| async move {
                    let library = ctx.library()?;
                    rename_file_path(
                        &library,
                        input.id,
                        input.is_dir,
                        &input.materialized_path,
                        &input.old_name,
                        &input.new_name,
                    )
                    .await?;
                    Ok(())
                }
            })
        })
        .mutation("move_file_path", |t| {
            t({
                #[derive(Deserialize, Type, Debug)]
                #[serde(rename_all = "camelCase")]
                struct FilePathMovePayload {
                    active: FilePathRequestPayload,
                    target: Option<FilePathRequestPayload>,
                }
                |ctx, input: FilePathMovePayload| async move {
                    let library = ctx.library()?;
                    move_file_path(&library, input.active, input.target).await?;
                    Ok(())
                }
            })
        })
        .mutation("delete_file_path", |t| {
            t({
                #[derive(Deserialize, Type, Debug)]
                #[serde(rename_all = "camelCase")]
                struct FilePathDeletePayload {
                    #[serde(deserialize_with = "validators::materialized_path_string")]
                    materialized_path: String,
                    #[serde(deserialize_with = "validators::path_name_string")]
                    name: String,
                }
                |ctx, input: FilePathDeletePayload| async move {
                    let library = ctx.library()?;
                    delete_file_path(&library, &input.materialized_path, &input.name).await?;
                    Ok(())
                }
            })
        })
        .mutation("process_video_asset", |t| {
            t(|ctx, input: i32| async move {
                let library = ctx.library()?;
                let file_path_id = input;
                process_video_asset(&library, &ctx, file_path_id).await?;
                Ok(())
            })
        })
        .mutation("process_video_metadata", |t| {
            t(|ctx, input: i32| async move {
                let library = ctx.library()?;
                let asset_object_id = input;
                process_video_metadata(&library, asset_object_id).await?;
                Ok(())
            })
        })
}
