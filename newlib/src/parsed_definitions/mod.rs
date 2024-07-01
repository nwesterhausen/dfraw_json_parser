//! These are the parsed results from reading the raw .txt files.

mod info_file;
mod steam_data;

pub mod body_size;
pub mod caste;
pub mod color;
pub mod creature;
pub mod gait;
pub mod milkable;
pub mod name;
pub mod select_creature;
pub mod state_names;
pub mod tags;
pub mod temperatures;
pub mod tile;

pub use info_file::InfoFile;
