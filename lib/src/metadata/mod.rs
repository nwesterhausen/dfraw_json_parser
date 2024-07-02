//! Metadata about the raw files and the raw objects themselves. Used for searching and other metadata-related tasks.

mod object_type;
mod parser_options;
mod raw_location;
mod raw_metadata;
mod token_complexity;

pub use object_type::ObjectType;
pub use object_type::OBJECT_TOKEN_MAP;
pub use parser_options::ParserOptions;
pub use raw_location::RawModuleLocation;
#[allow(clippy::module_name_repetitions)]
/// Metadata about the raw file
pub use raw_metadata::Metadata as RawMetadata;
/// The complexity of a raw object token
pub use token_complexity::TagComplexity;
