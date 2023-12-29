use serde::{Deserialize, Serialize};

use super::{Details, Task};

/// A payload that can be used to gauge the progress of the parsing process.
#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct ProgressPayload {
    /// The details of the current progress.
    pub details: Details,
    /// The current task.
    pub current_task: Task,
    /// The estimated percentage of completion.
    pub percentage: f64,
    /// The current total number of raws parsed.
    pub running_total: usize,
}
