use serde::{Deserialize, Serialize};

use super::RawModuleLocation;

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

impl DFInfoFile {
    pub fn new(id: &str, location: RawModuleLocation, parent_directory: &str) -> Self {
        Self {
            identifier: id.to_string(),
            location,
            parent_directory: parent_directory.to_owned(),
            numeric_version: 0,
            displayed_version: "0".to_string(),
            earliest_compatible_numeric_version: 0,
            earliest_compatible_displayed_version: "0".to_string(),
            author: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
        }
    }
    pub fn empty() -> Self {
        Self::new(
            "unknown",
            RawModuleLocation::Unknown,
            &String::from("none-specified"),
        )
    }
    pub fn get_identifier(&self) -> String {
        String::from(&self.identifier)
    }
    pub fn get_location(&self) -> RawModuleLocation {
        self.location
    }
    pub fn get_parent_directory(&self) -> String {
        String::from(&self.parent_directory)
    }
}
