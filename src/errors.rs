use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum DialettoError {
    #[error("Language file not found")]
    LanguageFileNotFound(#[from] io::Error),
    #[error("Encoding error")]
    EncodeError(#[from] bincode::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    // #[error("unknown data store error")]
    // Unknown,
}