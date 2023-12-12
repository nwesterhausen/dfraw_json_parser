use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub enum ProgressTask {
    ParseRaws,
    ParseLegends,
    ParseModuleInfoFiles,
    ResolveRaws,
    #[default]
    Idle,
}
