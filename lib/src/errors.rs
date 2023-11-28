use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Options validation error: {0}")]
    InvalidOptions(String),
    #[error("Invalid Dwarf Fortress raw file: {0}")]
    InvalidRawFile(String),
    #[error("Found unexpected object definition: {0}")]
    UnexpectedObjectType(String),
    #[error("")]
    Io {
        #[from]
        source: std::io::Error,
    },
}
