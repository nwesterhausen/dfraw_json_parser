use serde::{Deserialize, Serialize};

use crate::RawMetadata;

use super::Rule;

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreatureVariationRaw {
    /// Common Raw file Things
    #[serde(skip_serializing_if = "RawMetadata::is_hidden")]
    metadata: RawMetadata,
    identifier: String,
    object_id: String,

    /// Creature variations are basically just a set of simple tag actions which are applied to
    /// the creature which is being modified. The tags are applied in order EXCEPT for the convert
    /// tags which are applied in a reverse order.
    rules: Vec<Rule>,

    /// A creature variation can define any number of arguments which can be used in the rules.
    /// These arguments replace instances of `!ARGn` in the rules. Use `apply_arguments` to apply
    /// a set of arguments to a creature variation (and get a very specific variation back). Use
    /// `apply_to_creature` to apply the variation to a creature (it also takes arguments and will
    /// apply them to the variation before applying the variation to the creature).
    argument_count: usize,
}
