use serde::{Deserialize, Serialize};

use crate::parser::serializer_helper;

use super::tokens::CreatureEffectProperty;

/// A creature effect.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreatureEffect {
    severity: u32,
    probability: u8,

    affected_body_parts_by_category: Option<Vec<String>>,
    affected_body_parts_by_type: Option<Vec<String>>,
    affected_body_parts_by_token: Option<Vec<String>>,
    tags: Option<Vec<CreatureEffectProperty>>,

    start: u32,
    peak: u32,
    end: u32,

    dwf_stretch: Option<u8>,
}

impl CreatureEffect {
    /// Function to "clean" the creature. This is used to remove any empty list or strings,
    /// and to remove any default values. By "removing" it means setting the value to None.
    ///
    /// This also will remove the metadata if `is_metadata_hidden` is true.
    ///
    /// Steps:
    /// - Set any metadata to None if `is_metadata_hidden` is true.
    /// - Set any empty string to None.
    /// - Set any empty list to None.
    /// - Set any default values to None.
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        // Set any empty string to None.
        if let Some(affected_body_parts_by_category) =
            cleaned.affected_body_parts_by_category.clone()
        {
            if affected_body_parts_by_category.is_empty() {
                cleaned.affected_body_parts_by_category = None;
            }
        }

        // Set any empty string to None.
        if let Some(affected_body_parts_by_type) = cleaned.affected_body_parts_by_type.clone() {
            if affected_body_parts_by_type.is_empty() {
                cleaned.affected_body_parts_by_type = None;
            }
        }

        // Set any empty string to None.
        if let Some(affected_body_parts_by_token) = cleaned.affected_body_parts_by_token.clone() {
            if affected_body_parts_by_token.is_empty() {
                cleaned.affected_body_parts_by_token = None;
            }
        }

        // Set any empty string to None.
        if let Some(tags) = cleaned.tags.clone() {
            if tags.is_empty() {
                cleaned.tags = None;
            }
        }

        // Set any default values to None.
        if serializer_helper::is_zero_u8(cleaned.dwf_stretch) {
            cleaned.dwf_stretch = None;
        }

        cleaned
    }
}
