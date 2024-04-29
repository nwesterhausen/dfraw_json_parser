use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, specta::Type)]
#[serde(rename_all = "camelCase")]
#[derive(ts_rs::TS)]
#[ts(export)]
/// The additional data specific to the steam workshop
pub struct SteamData {
    title: Option<String>,
    description: Option<String>,
    tags: Option<Vec<String>>,
    key_value_tags: Option<Vec<String>>,
    metadata: Option<Vec<String>>,
    changelog: Option<String>,
    file_id: u64,
}

impl SteamData {
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.title.is_none()
            && self.description.is_none()
            && self.tags.is_none()
            && self.key_value_tags.is_none()
            && self.metadata.is_none()
            && self.changelog.is_none()
            && self.file_id == 0
    }
    // The various setters
    pub fn set_title(&mut self, title: &str) {
        self.title = Some(String::from(title));
    }
    pub fn set_description(&mut self, description: &str) {
        self.description = Some(String::from(description));
    }
    pub fn set_changelog(&mut self, changelog: &str) {
        self.changelog = Some(String::from(changelog));
    }
    pub fn set_file_id(&mut self, file_id: u64) {
        self.file_id = file_id;
    }
    // The various adders
    pub fn add_tag(&mut self, tag: &str) {
        if self.tags.is_none() {
            self.tags = Some(Vec::new());
        }

        if let Some(tags) = &mut self.tags {
            tags.push(String::from(tag));
        }
    }
    pub fn add_key_value_tag(&mut self, tag: &str) {
        if self.key_value_tags.is_none() {
            self.key_value_tags = Some(Vec::new());
        }

        if let Some(tags) = &mut self.key_value_tags {
            tags.push(String::from(tag));
        }
    }
    pub fn add_metadata(&mut self, metadata: &str) {
        if self.metadata.is_none() {
            self.metadata = Some(Vec::new());
        }

        if let Some(self_metadata) = &mut self.metadata {
            self_metadata.push(String::from(metadata));
        }
    }
}
