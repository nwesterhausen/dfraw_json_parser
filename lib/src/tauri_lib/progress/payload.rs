use serde::{Deserialize, Serialize};

use super::{Details, Task};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct ProgressPayload {
    pub details: Details,
    pub current_task: Task,
    pub percentage: f64,
    pub running_total: usize,
}
