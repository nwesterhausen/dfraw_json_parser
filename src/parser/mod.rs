/// Biome tokens and PHF map
pub(crate) mod biome;
/// Struct and parsing for body size tokens
pub(crate) mod body_size;
/// Struct and parsing for color tokens
pub(crate) mod color;
/// Struct and parsing for creature tokens. Also includes token enums and PHF maps.
pub(crate) mod creature;
/// Struct and parsing for creature caste tokens. Also includes token enums and PHF maps.
pub(crate) mod creature_caste;
/// Struct and parsing for creature effect tokens. Also includes token enums and PHF maps.
pub(crate) mod creature_effect;
/// Struct and parsing for creature variation tokens. Also includes token enums and PHF maps.
pub(crate) mod creature_variation;
/// Struct and parsing for entity tokens. Also includes token enums and PHF maps.
pub(crate) mod entity;
/// Struct and parsing for graphic tokens. Also includes token enums and PHF maps.
pub(crate) mod graphic;
/// Helper functions for parsing
pub(crate) mod helpers;
/// Handles parsing `info.txt` files into `ModuleInfoFile` structs
pub(crate) mod info_txt;
/// Struct and parsing for inorganic tokens. Also includes token enums and PHF maps.
pub(crate) mod inorganic;
/// Struct and parsing for material tokens. Also includes token enums and PHF maps.
pub(crate) mod material;
/// Struct for handling material mechanical properties.
pub(crate) mod material_mechanics;
/// Struct and parsing for material template tokens. Also includes token enums and PHF maps.
pub(crate) mod material_template;
/// Holds all the metadata for a raw -- i.e. information about where the raw was found, what module
/// it was in, etc.
pub(crate) mod metadata;
/// Struct and parsing for milkable tokens.
pub(crate) mod milkable;
/// Structs and parsing for the various name tokens.
pub(crate) mod names;
/// Enum of possible types of raw objects.
pub(crate) mod object_type;
/// Helper functions for parsing objects
pub(crate) mod parse;
/// Struct and parsing for plant tokens. Also includes token enums and PHF maps.
pub(crate) mod plant;
/// Struct and parsing for plant growth tokens. Also includes token enums and PHF maps.
pub(crate) mod plant_growth;
/// Struct and parsing for position tokens. Also includes token enums and PHF maps.
pub(crate) mod position;
/// Enum of the possible locations for raw modules.
pub(crate) mod raw_locations;
/// Holds some traits for raw objects
pub(crate) mod raws;
/// Reads raw files from disk and parses them into structs
pub(crate) mod reader;
/// Static pre-compiled regex and encoding data.
pub(crate) mod refs;
/// Searchable trait definition and helper functions
pub(crate) mod searchable;
/// Struct and parsing for seed material tokens. Also includes token enums and PHF maps.
pub(crate) mod seed_material;
/// Struct and parsing for select creature tokens. Also includes token enums and PHF maps.
pub(crate) mod select_creature;
/// Struct and parsing for shrub tokens. Also includes token enums and PHF maps.
pub(crate) mod shrub;
/// Struct and parsing for syndrome tokens. Also includes token enums and PHF maps.
pub(crate) mod syndrome;
/// Struct and parsing for temperature tokens.
pub(crate) mod temperature;
/// Struct and parsing for object tiles (how they're drawn without graphics)
pub(crate) mod tile;
/// Struct and parsing for tree tokens. Also includes token enums and PHF maps.
pub(crate) mod tree;
