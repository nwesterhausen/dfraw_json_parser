//! This library provides an API for parsing Dwarf Fortress raw files.

mod default_checks;
mod error;
mod legends_export;
mod parsed_definitions;
mod parser;

pub mod constants;
pub mod metadata;
pub mod raw_definitions;
pub mod regex;
pub mod traits;
pub mod utilities;

pub use error::Parser as ParserError;
pub use parsed_definitions::*;
