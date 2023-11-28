use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[derive(ts_rs::TS)]
#[ts(export)]
/// The additional data specific to the steam workshop
pub struct SteamData {
    #[serde(skip_serializing_if = "String::is_empty")]
    title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    key_value_tags: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    metadata: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    changelog: String,
    file_id: u64,
}

impl SteamData {
    pub fn is_empty(&self) -> bool {
        self.title.is_empty()
            && self.description.is_empty()
            && self.tags.is_empty()
            && self.key_value_tags.is_empty()
            && self.metadata.is_empty()
            && self.changelog.is_empty()
            && self.file_id == 0
    }
    // The various setters
    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }
    pub fn set_description(&mut self, description: &str) {
        self.description = String::from(description);
    }
    pub fn set_changelog(&mut self, changelog: &str) {
        self.changelog = String::from(changelog);
    }
    pub fn set_file_id(&mut self, file_id: u64) {
        self.file_id = file_id;
    }
    // The various adders
    pub fn add_tag(&mut self, tag: &str) {
        self.tags.push(String::from(tag));
    }
    pub fn add_key_value_tag(&mut self, tag: &str) {
        self.key_value_tags.push(String::from(tag));
    }
    pub fn add_metadata(&mut self, metadata: &str) {
        self.metadata.push(String::from(metadata));
    }
}
