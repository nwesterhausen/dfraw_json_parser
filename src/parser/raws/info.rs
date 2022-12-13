use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DFInfoFile {
    identifier: String,
    pub numeric_version: u32,
    pub displayed_version: String,
    pub earliest_compatible_numeric_version: u32,
    pub earliest_compatible_displayed_version: String,
    pub author: String,
    pub name: String,
    pub description: String,
}

impl DFInfoFile {
    pub fn new(id: &str) -> Self {
        Self {
            identifier: id.to_string(),
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
}
