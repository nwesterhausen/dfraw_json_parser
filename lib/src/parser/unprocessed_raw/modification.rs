use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum Modification {
    /// `COPY_TAGS_FROM` tag
    CopyTagsFrom {
        /// The creature to copy tags from
        identifier: String,
    },
    /// `APPLY_CREATURE_VARIATION` tag
    ApplyCreatureVariation {
        /// The creature to apply the variation from
        identifier: String,
    },
    /// Follows `GO_TO_END` until `GO_TO_START` or object definition finishes
    ///
    /// When using tags from an existing creature, inserts new tags at the end of the creature.
    AddToEnding {
        /// The set of raws to add to the end of the object
        ///
        /// This should be the entire raw in order to apply.
        raws: Vec<String>,
    },
    /// Follows `GO_TO_START` until `GO_TO_END` or object definition finishes
    ///
    /// When using tags from an existing creature, inserts new tags at the beginning of the creature.
    AddToBeginning {
        /// The set of raws to add to the beginning of the object
        ///
        /// This should be the entire raw in order to apply.
        raws: Vec<String>,
    },
    /// `GO_TO_TAG:tag` raw instruction
    ///
    /// When using tags from an existing creature, inserts new tags before the specified tag.
    AddBeforeTag {
        /// The tag to insert before
        ///
        /// Since we don't actually know the tag order after parsing, this will be ignored in parsing, and
        /// instead will just apply the raws...
        tag: String,
        /// The set of raws to add before the tag
        ///
        /// This should be the entire raw in order to apply.
        raws: Vec<String>,
    },
    MainRawBody {
        /// The set of raws that make up the object. This is usually defined first unless
        /// its specified to be added to the end or beginning (or before a tag)
        ///
        /// This should be the entire raw in order to apply.
        raws: Vec<String>,
    },
}

impl Modification {
    pub(crate) fn add_raw(&mut self, format: String) {
        match self {
            Modification::AddToEnding { raws }
            | Modification::AddToBeginning { raws }
            | Modification::AddBeforeTag { raws, .. }
            | Modification::MainRawBody { raws } => raws.push(format),
            _ => {}
        }
    }
}
