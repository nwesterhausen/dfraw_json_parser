use serde::{Deserialize, Serialize};

#[cfg(feature = "tauri")]
use tauri::Manager;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[derive(ts_rs::TS)]
#[ts(export)]
/// It's a struct to represent the progress of the current job. This is emitted back to the Tauri app using the `PROGRESS` event.
///
/// Properties:
///
/// * `percentage`: The percentage of completed steps out of total steps.
/// * `current_module`: The name of the module that is currently being processed.
pub struct ProgressPayload {
    pub current_task: String,
    pub percentage: f64,
    pub current_module: String,
    pub current_file: String,
    pub current_location: String,
    pub running_total: usize,
    #[serde(skip)]
    pub total_steps: usize,
    #[serde(skip)]
    pub current_step: usize,
}

#[cfg(feature = "tauri")]
pub struct ProgressHelper {
    tauri_window: tauri::Window,
}

#[cfg(feature = "tauri")]
impl ProgressHelper {
    pub fn with_tauri_window(window: tauri::Window) -> Self {
        Self {
            tauri_window: window,
        }
    }
    pub fn send(&self, progress_cache: &ProgressPayload) {
        if let Err(e) = self.tauri_window.emit("PROGRESS", &progress_cache) {
            log::debug!("Tauri window emit error {:?}", e);
        };
    }
}
impl ProgressPayload {
    #[allow(clippy::cast_precision_loss)]
    pub fn add_steps(&mut self, amount: usize) {
        self.total_steps += amount;
        self.percentage = self.current_step as f64 / self.total_steps as f64;
    }
    pub fn update_current_location(&mut self, location: &str) {
        self.current_location = String::from(location);
    }
    pub fn update_current_module(&mut self, module: &str) {
        self.current_module = String::from(module);
    }
    pub fn update_current_task(&mut self, task: &str) {
        self.current_task = String::from(task);
    }
    pub fn add_to_running_total(&mut self, amount: usize) {
        self.running_total += amount;
    }
    pub fn set_current_file(&mut self, file: &str) {
        self.current_file = String::from(file);
    }
    #[allow(clippy::cast_precision_loss)]
    pub fn step_advance(&mut self) {
        self.current_step += 1;
        self.percentage = self.current_step as f64 / self.total_steps as f64;
    }
    pub fn set_final(&mut self, message: &str) {
        self.current_file = String::from("None");
        self.current_module = String::from("None");
        self.current_task = String::from(message);
        self.percentage = 1.0;
    }
}
