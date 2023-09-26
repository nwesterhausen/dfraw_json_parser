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
        if CV_TOKENS.contains_key(key) {
            CV_TOKENS.get(key).unwrap().clone()
        } else {
            log::error!("Invalid Creature Variation key: {}", key);
            CVTag::Unknown
        }
    }
}
