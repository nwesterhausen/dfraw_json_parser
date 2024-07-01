//! This library provides an API for parsing Dwarf Fortress raw files.

mod error;
mod legends_export;
mod parsed_definitions;
mod parser;
mod raw_definitions;
mod utilities;

pub mod constants;
pub mod metadata;
pub mod regex;
pub mod traits;

pub use error::Parser as ParserError;
pub use parsed_definitions::*;
