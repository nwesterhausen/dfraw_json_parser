use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub enum SelectRules {
    /// Selects a previously defined caste
    SelectCaste(String),
    /// Selects a locally defined material. Can be ALL.
    SelectMaterial(String),
    /// Selects a tissue for editing.
    SelectTissue(String),
    /// Adds an additional previously defined caste to the selection. Used after [SELECT_CASTE].
    SelectAdditionalCaste(String),
}
