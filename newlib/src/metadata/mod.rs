//! Metadata about the raw files and the raw objects themselves. Used for searching and other metadata-related tasks.

mod object;
mod object_type;
mod parser_options;
mod raw_location;
mod raw_metadata;
mod searchable;
mod token;

/// A Dwarf Fortress raw object
pub use object::RawObject;
/// Allows conversion from a raw object to any specific object
pub use object::RawObjectToAny;
pub use object_type::ObjectType;
pub use parser_options::ParserOptions;
pub use raw_location::RawModuleLocation;
#[allow(clippy::module_name_repetitions)]
/// Metadata about the raw file
pub use raw_metadata::Metadata as RawMetadata;
/// Prepares the data from the raw object to be turned into a search string
pub use searchable::clean_search_vec;
/// Gets the search string from the raw object
pub use searchable::get_search_string;
/// Provides the search functionality for the raw object
pub use searchable::Searchable;
/// The complexity of a raw object token
pub use token::Complexity as TokenComplexity;
