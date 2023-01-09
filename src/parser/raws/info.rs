use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DFInfoFile {
    identifier: String,
    sourced_directory: String,
    relative_path: String,
    pub numeric_version: u32,
    pub displayed_version: String,
    pub earliest_compatible_numeric_version: u32,
    pub earliest_compatible_displayed_version: String,
    pub author: String,
    pub name: String,
    pub description: String,
}

impl DFInfoFile {
    pub fn new(id: &str, sourced_dir: &str, relative_path: &String) -> Self {
        Self {
            identifier: id.to_string(),
            sourced_directory: sourced_dir.to_string(),
            relative_path: relative_path.clone(),
            numeric_version: 0,
            displayed_version: "0".to_string(),
            earliest_compatible_numeric_version: 0,
            earliest_compatible_displayed_version: "0".to_string(),
            author: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
        }
    }

    pub fn get_identifier(&self) -> String {
        String::from(&self.identifier)
    }
    pub fn get_sourced_directory(&self) -> String {
        String::from(&self.sourced_directory)
    }
    pub fn get_relative_path(&self) -> String {
        String::from(&self.relative_path)
    }
}
