use serde::{Deserialize, Serialize};

use super::phf_table::CV_TOKENS;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum CVTag {
    NewTag,
    AddTag,
    RemoveTag,
    ConvertTag,
    ConvertTagMaster,
    ConvertTagTarget,
    ConvertTagReplacement,
    ConditionalNewTag,
    ConditionalAddTag,
    ConditionalRemoveTag,
    ConditionalConvertTag,
    #[default]
    Unknown,
}

impl CVTag {
    pub fn from_key(key: &str) -> Self {
        let tag = CV_TOKENS.get(key).unwrap_or(&CVTag::Unknown);
        if tag == &CVTag::Unknown {
            log::warn!("Unknown creature variation (CV) tag: {}", key);
        }
        tag.clone()
    }
}
