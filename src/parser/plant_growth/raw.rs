use serde::{Deserialize, Serialize};

use crate::parser::names::Name;

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct PlantGrowth {
    identifier: String,
    name: Name,
}
