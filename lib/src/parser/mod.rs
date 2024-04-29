pub mod biome;
pub mod creature;
pub mod creature_caste;
pub mod creature_effect;
pub mod creature_variation;
pub mod entity;
pub mod graphics;
pub mod helpers;
pub mod inorganic;
pub mod material;
pub mod material_template;
pub mod plant;
pub mod plant_growth;
pub mod position;
pub mod seed_material;
pub mod select_creature;
pub mod shrub;
pub mod syndrome;
pub mod tree;
pub mod unprocessed_raw;

mod constants;
mod material_mechanics;
mod metadata;
mod module_info_file;
mod names;
mod object_types;
mod raw_locations;
mod reader;
pub(crate) mod serializer_helper;
mod simple_structs;

pub use constants::*;
/// Represents the mechanical properties of a material
pub use material_mechanics::MaterialMechanics;
/// The breakdown of a material's mechanical properties
pub use material_mechanics::MechanicalProperties;
/// Cleans a search vector by removing duplicates and sorting it
pub use metadata::clean_search_vec;
/// Gets the search string for a searchable object
pub use metadata::get_search_string;
/// Metadata about a raw file
pub use metadata::RawMetadata;
/// Represents a raw object in DF Raws
pub use metadata::RawObject;
/// Trait used to convert a raw object to a specific type
pub use metadata::RawObjectToAny;
/// Trait used to make an object searchable
pub use metadata::Searchable;
/// A module info file is the `info.txt` file in the module directory.
pub use module_info_file::ModuleInfoFile;
/// The Steam data included in the module info file
pub use module_info_file::SteamData;
pub use names::*;
/// Represents the type of an object in DF Raws
pub use object_types::ObjectType;
/// An efficient map of the string tokens to their enum values
pub use object_types::OBJECT_TOKEN_MAP;
/// Standard location options for raw files in the DF Directory
pub use raw_locations::RawModuleLocation;
/// The results of parsing a raw file
pub use reader::FileParseResults;
/// List of the only object types that can be parsed right now
pub use reader::PARSABLE_OBJECT_TYPES;
pub use simple_structs::*;

pub(crate) use reader::parse_raw_file;
