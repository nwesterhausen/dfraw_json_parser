use serde::{Deserialize, Serialize};

use crate::parser::serializer_helper;

use super::tokens::CreatureEffectProperty;

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreatureEffect {
    severity: u32,
    probability: u8,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    affected_body_parts_by_category: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    affected_body_parts_by_type: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    affected_body_parts_by_token: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<CreatureEffectProperty>,
    start: u32,
    peak: u32,
    end: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u8")]
    dwf_stretch: u8,
}
