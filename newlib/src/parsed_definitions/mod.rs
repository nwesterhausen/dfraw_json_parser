//! These are the parsed results from reading the raw .txt files.

mod tag_parsing;

pub mod body_size;
pub mod caste;
pub mod color;
pub mod creature;
pub mod creature_effect;
pub mod creature_variation;
pub mod custom_graphic_extension;
pub mod dimensions;
pub mod entity;
pub mod gait;
pub mod graphic;
pub mod info_file;
pub mod inorganic;
pub mod material;
pub mod material_mechanics;
pub mod material_template;
pub mod mechanical_properties;
pub mod milkable;
pub mod name;
pub mod plant;
pub mod plant_growth;
pub mod position;
pub mod seed_material;
pub mod select_creature;
pub mod shrub;
pub mod sprite_graphic;
pub mod sprite_layer;
pub mod state_names;
pub mod steam_data;
pub mod syndrome;
pub mod tags;
pub mod temperatures;
pub mod tile;
pub mod tile_page;
pub mod tree;

pub use info_file::InfoFile;
