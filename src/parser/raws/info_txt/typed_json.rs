use crate::parser::{
    raws::{info_txt::DFInfoFile, RawModuleLocation},
    TypedJsonSerializable,
};
use serde::{Deserialize, Serialize};
use slug::slugify;

// Info file Object for Web Consumption
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TypedJson {
    identifier: String,
    sourced_directory: RawModuleLocation,
    numeric_version: u32,
    displayed_version: String,
    earliest_compatible_numeric_version: u32,
    earliest_compatible_displayed_version: String,
    author: String,
    name: String,
    description: String,
    display_title: String,
    relative_path: String,
    object_id: String,
}

impl TypedJson {
    pub fn from(info_file: &DFInfoFile) -> Self {
        Self {
            author: info_file.author.to_string(),
            description: info_file.description.to_string(),
            displayed_version: info_file.displayed_version.to_string(),
            earliest_compatible_displayed_version: info_file
                .earliest_compatible_displayed_version
                .to_string(),
            earliest_compatible_numeric_version: info_file.earliest_compatible_numeric_version,
            identifier: info_file.get_identifier(),
            name: info_file.name.to_string(),
            numeric_version: info_file.numeric_version,
            sourced_directory: info_file.get_location(),
            display_title: format!("{} v{}", info_file.name, info_file.displayed_version),
            object_id: format!(
                "{}_{}_{}",
                slugify(&info_file.author),
                info_file.get_identifier(),
                info_file.numeric_version
            ),
            relative_path: info_file.get_parent_directory(),
        }
    }
}

impl TypedJsonSerializable for DFInfoFile {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&TypedJson::from(self))
    }
}
