use crate::parser::{
    raws::{info_txt::DFInfoFile, RawModuleLocation},
    TypedJsonSerializable,
};
use serde::{Deserialize, Serialize};
use slug::slugify;

// Info file Object for Web Consumption
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypedJson {
    identifier: String,
    #[serde(rename = "sourcedDirectory")]
    sourced_directory: RawModuleLocation,
    #[serde(rename = "numericVersion")]
    numeric_version: u32,
    #[serde(rename = "displayedVersion")]
    displayed_version: String,
    #[serde(rename = "earliestCompatibleNumericVersion")]
    earliest_compatible_numeric_version: u32,
    #[serde(rename = "earliestCompatibleDisplayedVersion")]
    earliest_compatible_displayed_version: String,
    author: String,
    name: String,
    description: String,
    #[serde(rename = "displayTitle")]
    display_title: String,
    #[serde(rename = "relativePath")]
    relative_path: String,
    #[serde(rename = "objectId")]
    object_id: String,
}

impl TypedJson {
    pub fn from(info_file: &DFInfoFile) -> Self {
        Self {
            author: info_file.author.clone(),
            description: info_file.description.clone(),
            displayed_version: info_file.displayed_version.clone(),
            earliest_compatible_displayed_version: info_file
                .earliest_compatible_displayed_version
                .clone(),
            earliest_compatible_numeric_version: info_file.earliest_compatible_numeric_version,
            identifier: info_file.get_identifier(),
            name: info_file.name.clone(),
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
