use thiserror::Error;

/// Represents an error that occurred while parsing a raw file
#[derive(Error, Debug)]
pub enum Parser {
    /// An invalid token was found
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    /// An invalid option was provided
    #[error("Options validation error: {0}")]
    InvalidOptions(String),
    /// An invalid raw file was found
    #[error("Invalid Dwarf Fortress raw file: {0}")]
    InvalidRawFile(String),
    /// An invalid object was found
    #[error("Found unexpected object definition: {0}")]
    UnexpectedObjectType(String),
    /// An IO error occurred
    #[error("")]
    Io {
        /// The source of the error
        #[from]
        source: std::io::Error,
    },
    /// Not yet implemented! error
    #[error("Not yet implemented (TODO)")]
    NotYetImplemented,
    /// Cannot parse the target from the given string
    #[error("Target for parsing cannot be opened: {0}")]
    NothingToParse(String),
}
