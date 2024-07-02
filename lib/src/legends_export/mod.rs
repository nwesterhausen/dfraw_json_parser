//! This module supports parsing Dwarf Fortress legends export files.
//!
//! So far this only supports parsing the creature and entity sections of the legends export.

mod reader;
mod util;

pub mod exports;

pub use exports::ExportedCreature;
pub use exports::ExportedEntity;
pub use reader::parse_legends_export as parse;
