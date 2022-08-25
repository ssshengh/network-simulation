use std::io;
use thiserror::Error;

pub type NResult<T> = Result<T, ThreadError>;

#[derive(Error, Debug)]
pub enum ThreadError {
    #[error("The thread spawned by os face unexpected situation.")]
    OSFailToCreateThread(#[from] io::Error),
    #[error("The task sent by thread pool failure. Flume error: {0}")]
    TaskSendError(String),
}
