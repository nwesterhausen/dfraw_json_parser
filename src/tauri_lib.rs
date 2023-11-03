use serde::{Deserialize, Serialize};

#[cfg(feature = "tauri")]
use tauri::Manager;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[derive(ts_rs::TS)]
#[ts(export)]
/// A struct to represent the progress of the current job. This is emitted back to the Tauri app using the `PROGRESS` event.
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

/// A helper struct to send progress updates back to the Tauri app. This is
/// only used when the `tauri` feature is enabled. It is a wrapper around the
/// `tauri::Window` so that we don't have to `use tauri::Window` in our parsing
/// modules (because that would require the `tauri` feature to be enabled).
///
/// While this also requires the feature, because we can `use ProgressHelper`
/// without having to import the entire tauri library when we have no tauri feature
/// flag lets us continue to only pull in the tauri library when we need it.
///
/// The implementations of this struct are used to call `emit` on the `tauri::Window`
pub struct ProgressHelper {
    #[cfg(feature = "tauri")]
    /// The tauri window to emit the progress event to.
    tauri_window: tauri::Window,
}

impl ProgressHelper {
    #[cfg(feature = "tauri")]
    /// Create a new `ProgressHelper` with the given `tauri::Window`.
    pub fn with_tauri_window(window: tauri::Window) -> Self {
        Self {
            tauri_window: window,
        }
    }
    #[cfg(feature = "tauri")]
    /// Send the given `ProgressPayload` to the tauri window.
    ///
    /// This will emit the `PROGRESS` event to the tauri window.
    ///
    /// Arguments:
    ///
    /// * `progress_cache`: The `ProgressPayload` to send to the tauri window.
    pub fn send(&self, progress_cache: &ProgressPayload) {
        if let Err(e) = self.tauri_window.emit("PROGRESS", &progress_cache) {
            log::debug!("Tauri window emit error {:?}", e);
        };
    }
}

impl ProgressPayload {
    #[allow(clippy::cast_precision_loss)]
    /// Add the given amount to the total steps and update the percentage.
    ///
    /// Arguments:
    ///
    /// * `amount`: The amount to add to the total steps.
    pub fn add_steps(&mut self, amount: usize) {
        self.total_steps += amount;
        self.percentage = self.current_step as f64 / self.total_steps as f64;
    }
    /// Set the `current_location`
    ///
    /// Arguments:
    ///
    /// * `location`: The location to set the `current_location` to.
    pub fn update_current_location(&mut self, location: &str) {
        self.current_location = String::from(location);
    }
    /// Set the `current_module`
    ///
    /// Arguments:
    ///
    /// * `module`: The module to set the `current_module` to.
    pub fn update_current_module(&mut self, module: &str) {
        self.current_module = String::from(module);
    }
    /// Set the `current_task`
    ///
    /// Todo: This should be required to use an enum of possible tasks instead
    ///
    /// Arguments:
    ///
    /// * `task`: The task to set the `current_task` to.
    pub fn update_current_task(&mut self, task: &str) {
        self.current_task = String::from(task);
    }
    /// Set the `current_file`
    ///
    /// Arguments:
    ///
    /// * `file`: The file to set the `current_file` to.
    pub fn add_to_running_total(&mut self, amount: usize) {
        self.running_total += amount;
    }
    /// Set the `current_file`
    ///
    /// Arguments:
    ///
    /// * `file`: The file to set the `current_file` to.
    pub fn set_current_file(&mut self, file: &str) {
        self.current_file = String::from(file);
    }
    #[allow(clippy::cast_precision_loss)]
    /// Increment the current step and update the percentage.
    ///
    /// This can be called instead of `add_steps` if you only want to increment
    /// the current step by one.
    pub fn step_advance(&mut self) {
        self.current_step += 1;
        self.percentage = self.current_step as f64 / self.total_steps as f64;
    }
    /// Indicate that the job is complete.
    ///
    /// This will set the `current_file`, and `current_module` to "None" and set the percentage to 1.0.
    ///
    /// Arguments:
    ///
    /// * `message`: The message to set the `current_task` to.
    pub fn set_final(&mut self, message: &str) {
        self.current_file = String::from("None");
        self.current_module = String::from("None");
        self.current_task = String::from(message);
        self.percentage = 1.0;
    }
}
