use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Hash)]
pub enum Error {
    #[error("Other Error: {0}")]
    Other(String),
    #[error("Error getting path: {0}")]
    Path(String),
    #[error("Error getting file data: {0}")]
    File(String),
    #[error("Error parsing file: {0}")]
    Parse(String),
}