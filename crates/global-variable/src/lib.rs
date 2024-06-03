use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};
use storage::Storage;

// Map<root_path, Storage>
pub static STORAGE_MAP: OnceLock<Arc<RwLock<HashMap<String, Storage>>>> = OnceLock::new();

pub static CURRENT_LIBRARY_DIR: OnceLock<Arc<RwLock<String>>> = OnceLock::new();

pub fn init_storage_map() -> Arc<RwLock<HashMap<String, Storage>>> {
    Arc::new(RwLock::new(HashMap::new()))
}

#[macro_export]
macro_rules! init_current_library_dir {
    () => {{
        std::sync::Arc::new(std::sync::RwLock::new("".to_string()))
    }};
}

#[macro_export]
macro_rules! init_global_variables {
    () => {
        $crate::STORAGE_MAP.get_or_init(|| $crate::init_storage_map());
        $crate::CURRENT_LIBRARY_DIR.get_or_init(|| $crate::init_current_library_dir!());
    };
}

#[macro_export]
macro_rules! read_storage_map {
    () => {{
        $crate::STORAGE_MAP
            .get_or_init(|| $crate::init_storage_map())
            .read()
            .map_err(|_| StorageError::MutexPoisonError("Fail to read storage map".to_string()))
    }};
}

#[macro_export]
macro_rules! write_storage_map {
    () => {{
        $crate::STORAGE_MAP
            .get_or_init(|| $crate::init_storage_map())
            .write()
            .map_err(|e| anyhow::anyhow!("Could not write storage map: {e}"))
    }};
}

#[macro_export]
macro_rules! read_current_library_dir {
    () => {{
        let current_library_dir =
            $crate::CURRENT_LIBRARY_DIR.get_or_init(|| $crate::init_current_library_dir!());
        current_library_dir.read().map_err(|e| {
            StorageError::MutexPoisonError("Fail to read current library dir".to_string())
        })
    }};
}

#[macro_export]
macro_rules! write_current_library_dir {
    () => {{
        let current_library_dir =
            $crate::CURRENT_LIBRARY_DIR.get_or_init(|| $crate::init_current_library_dir!());
        current_library_dir
            .write()
            .map_err(|e| anyhow::anyhow!("Could not write current library dir: {e}"))
    }};
}

#[macro_export]
macro_rules! get_current_storage {
    () => {{
        let current_library_dir = $crate::read_current_library_dir!().unwrap().clone();
        let map = $crate::read_storage_map!().unwrap();
        map.get(&current_library_dir)
            .map(|s| s.clone())
            .ok_or_else(|| StorageError::NoStorageFound)
    }};
}

#[macro_export]
macro_rules! set_storage {
    (library_dir = $dir:expr, storage = $storage:expr) => {{
        let map = $crate::STORAGE_MAP.get_or_init(|| init_storage_map());
        let mut write_guard = map
            .write()
            .map_err(|e| anyhow::anyhow!("Could not write storage map: {e}"))?;
        write_guard.insert(library_dir, storage);
        Ok(())
    }};
}

#[macro_export]
macro_rules! set_current_library_dir {
    ($dir:expr) => {{
        match $crate::write_current_library_dir!() {
            Ok(mut current_library_dir) => {
                *current_library_dir = $dir;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }};
}
