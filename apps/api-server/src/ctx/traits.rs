use crate::{ai::AIHandler, task_queue::TaskPayload};
use content_library::Library;
use file_handler::video::{VideoHandler, VideoTaskType};
use std::{
    boxed::Box,
    path::PathBuf,
    pin::Pin,
    sync::{mpsc::Sender, Arc, Mutex},
};

#[derive(Debug)]
pub struct StoreError(pub String);

pub trait CtxStore {
    fn load(&mut self) -> Result<(), StoreError>;
    fn save(&self) -> Result<(), StoreError>;
    fn insert(&mut self, key: &str, value: &str) -> Result<(), StoreError>;
    fn get(&self, key: &str) -> Option<String>;
    fn delete(&mut self, key: &str) -> Result<(), StoreError>;
}

pub trait CtxWithLibrary {
    fn get_local_data_root(&self) -> PathBuf;
    fn get_resources_dir(&self) -> PathBuf;

    fn switch_current_library<'async_trait>(
        &'async_trait self,
        library_id: &'async_trait str,
    ) -> Pin<Box<dyn std::future::Future<Output = ()> + Send + 'async_trait>>
    where
        Self: Sync + 'async_trait;

    fn library(&self) -> Result<Library, rspc::Error>;

    fn get_task_tx(&self) -> Arc<Mutex<Sender<TaskPayload<VideoHandler, VideoTaskType>>>>;
    fn get_ai_handler(&self) -> AIHandler;
}
