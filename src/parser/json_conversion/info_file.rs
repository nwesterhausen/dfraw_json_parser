use super::TypedJsonSerializable;
use crate::parser::raws::info::DFInfoFile;
use serde::{Deserialize, Serialize};

// Info file Object for Web Consumption
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypedJsonInfoFile {
    identifier: String,
    sourced_directory: String,
    numeric_version: u32,
    displayed_version: String,
    earliest_compatible_numeric_version: u32,
    earliest_compatible_displayed_version: String,
    author: String,
    name: String,
    description: String,
}

impl TypedJsonInfoFile {
    pub fn from(info_file: &DFInfoFile) -> Self {
        Self {
            author: info_file.author.to_owned(),
            description: info_file.description.to_owned(),
            displayed_version: info_file.displayed_version.to_owned(),
            earliest_compatible_displayed_version: info_file
                .earliest_compatible_displayed_version
                .to_owned(),
            earliest_compatible_numeric_version: info_file.earliest_compatible_numeric_version,
            identifier: info_file.get_identifier(),
            name: info_file.name.to_owned(),
            numeric_version: info_file.numeric_version,
            sourced_directory: info_file.get_sourced_directory(),
        }
    }
}

impl TypedJsonSerializable for DFInfoFile {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&TypedJsonInfoFile::from(&self))
    }
}
