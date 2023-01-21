use serde::{Deserialize, Serialize};

use super::RawModuleLocation;

mod impl_basic;
mod parse;
mod typed_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct DFInfoFile {
    identifier: String,
    location: RawModuleLocation,
    parent_directory: String,
    pub numeric_version: u32,
    pub displayed_version: String,
    pub earliest_compatible_numeric_version: u32,
    pub earliest_compatible_displayed_version: String,
    pub author: String,
    pub name: String,
    pub description: String,
}
