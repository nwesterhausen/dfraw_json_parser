mod refs;
mod regex;

/// The file encoding supported by dwarf fortress
pub use refs::DF_ENCODING;
/// Static regex to re-use instead of re-make each time they're needed
pub use regex::*;
