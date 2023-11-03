/// The perfect hash function table to connect the string literal token to the enum variant.
mod phf_table;
/// Contains the raw struct and parsing functions for the caste tokens.
mod raw;
/// Contains the enum for the caste tokens.
mod tokens;

pub use phf_table::CASTE_TOKENS;
pub use raw::Caste;
pub use tokens::CasteTag;
