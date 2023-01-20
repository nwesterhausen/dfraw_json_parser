use crate::parser::raws::RawModuleLocation;

impl super::DFInfoFile {
    pub fn new(id: &str, location: RawModuleLocation, parent_directory: &str) -> Self {
        Self {
            identifier: id.to_string(),
            location,
            parent_directory: parent_directory.to_owned(),
            numeric_version: 0,
            displayed_version: "0".to_string(),
            earliest_compatible_numeric_version: 0,
            earliest_compatible_displayed_version: "0".to_string(),
            author: String::new(),
            name: String::new(),
            description: String::new(),
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
