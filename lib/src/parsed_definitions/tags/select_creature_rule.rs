//! The rules for selecting a creature

/// The rules for selecting a creature
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, specta::Type)]
pub enum SelectCreatureRuleTag {
    /// Selects a previously defined caste
    SelectCaste(String),
    /// Selects a locally defined material. Can be ALL.
    SelectMaterial(String),
    /// Selects a tissue for editing.
    SelectTissue(String),
    /// Adds an additional previously defined caste to the selection. Used after `[SELECT_CASTE]`.
    SelectAdditionalCaste(String),
}
