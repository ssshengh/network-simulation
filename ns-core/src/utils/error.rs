use thiserror::Error;

type Result<T> = std::result::Result<T, NsError>;

#[derive(Debug, Error)]
pub enum NsError {}
