use serde::{Deserialize, Serialize};

use crate::RawModuleLocation;

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct ProgressDetails {
    pub location: RawModuleLocation,
    pub module: Option<String>,
    pub raw_file: Option<String>,
    pub file_location: Option<String>,
}
