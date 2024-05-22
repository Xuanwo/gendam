use opendal::Error as OpenDalError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, StorageError>;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Storage error: {0}")]
    OpenDalError(#[from] OpenDalError),

    #[error("Storage unexpected error")]
    UnexpectedError,

    #[error("Storage tokio fs error: {0}")]
    TokioFsError(#[from] tokio::io::Error),
}
