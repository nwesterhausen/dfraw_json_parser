use serde::{Deserialize, Serialize};

/// The rules for selecting a creature
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, specta::Type)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub enum SelectRules {
    /// Selects a previously defined caste
    SelectCaste(String),
    /// Selects a locally defined material. Can be ALL.
    SelectMaterial(String),
    /// Selects a tissue for editing.
    SelectTissue(String),
    /// Adds an additional previously defined caste to the selection. Used after `[SELECT_CASTE]`.
    SelectAdditionalCaste(String),
}
