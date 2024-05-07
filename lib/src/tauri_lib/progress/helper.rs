use crate::RawModuleLocation;

use super::{Details, Payload, Task};

#[cfg(feature = "tauri")]
#[allow(clippy::module_name_repetitions)]
/// A helper struct for sending progress updates to the Tauri frontend.
///
/// This struct is only available when the `tauri` feature is enabled.
pub struct ProgressHelper {
    /// The estimated total number of steps that will be taken.
    estimated_total: usize,
    /// The current step index.
    current_index: usize,
    /// The Tauri window to send updates to.
    tauri_window: tauri::Window,
    /// The progress cache.
    progress_cache: Payload,
}

#[cfg(feature = "tauri")]
impl ProgressHelper {
    /// Create a new `ProgressHelper` with the given Tauri window.
    ///
    /// # Parameters
    ///
    /// * `window` - The Tauri window to send updates to.
    pub fn with_tauri_window(window: tauri::Window) -> Self {
        Self {
            estimated_total: 1,
            current_index: 0,
            tauri_window: window,
            progress_cache: Payload::default(),
        }
    }
    /// Add the given number of steps to the estimated total.
    ///
    /// # Parameters
    ///
    /// * `amount` - The number of steps to add.
    pub fn add_steps(&mut self, amount: usize) {
        self.estimated_total += amount;
    }
    /// Reset the details of the progress.
    ///
    /// This will reset the module, file, file location, and module location.
    pub fn reset_details(&mut self) {
        self.progress_cache.details = Details::default();
    }
    /// Set the current task.
    ///
    /// # Parameters
    ///
    /// * `task` - The current task.
    pub fn set_task(&mut self, task: Task) {
        self.progress_cache.current_task = task;
    }
    /// Set the module location being processed.
    ///
    /// # Parameters
    ///
    /// * `location` - The module location within the Dwarf Fortress directory.
    pub fn set_location(&mut self, location: RawModuleLocation) {
        self.progress_cache.details.location = location;
    }
    /// Set the name of the module currently being processed.
    ///
    /// # Parameters
    ///
    /// * `module` - The name of the module currently being processed.
    pub fn set_module(&mut self, module: &str) {
        self.progress_cache.details.module = Some(String::from(module));
    }
    /// Set the name of the file currently being processed.
    ///
    /// # Parameters
    ///
    /// * `file` - The name of the file currently being processed.
    pub fn set_file(&mut self, file: &str) {
        self.progress_cache.details.raw_file = Some(String::from(file));
    }
    /// Set the file location of the file currently being processed.
    ///
    /// # Parameters
    ///
    /// * `file_location` - The file location of the file currently being processed.
    pub fn set_file_location(&mut self, file_location: &str) {
        self.progress_cache.details.file_location = Some(String::from(file_location));
    }
    /// Add some amount of progress to the running total.
    ///
    /// This should be tracking the estimated number of raws and then this helps calculate the percentage.
    ///
    /// # Parameters
    ///
    /// * `amount` - The amount of progress to add to the running total.
    pub fn add_to_running_total(&mut self, amount: usize) {
        self.progress_cache.running_total += amount;
    }
    /// Advance the current step index.
    ///
    /// This can be called after each step for convenience. It adds one to the current step index and updates the percentage.
    #[allow(clippy::cast_precision_loss)]
    fn step_advance(&mut self) {
        self.current_index += 1;
        self.progress_cache.percentage = self.current_index as f64 / self.estimated_total as f64;
    }
    /// Advance the current step index and send an update to the Tauri frontend.
    pub fn advance_and_send_update(&mut self) {
        self.step_advance();
        self.send_update();
    }
    /// Send an update to the Tauri frontend with the current progress.
    pub fn send_update(&mut self) {
        use tauri::Manager;

        self.tauri_window
            .emit("progress", self.progress_cache.clone())
            .unwrap_or_else(|e| tracing::debug!("Tauri window emit error {:?}", e));
    }
    /// Update the current task/details/index/percentage to values that would be expected for the end of the process.
    ///
    /// This should be called when the process is complete. It updates all detail values to `None` and sets the current task to `Task::Idle`.
    ///
    /// This also sets the percentage to 1.0 and the current index to the estimated total.
    pub fn finalize(&mut self) {
        self.progress_cache.details.raw_file = None;
        self.progress_cache.details.file_location = None;
        self.progress_cache.details.module = None;
        self.progress_cache.details.location = RawModuleLocation::Unknown;
        self.progress_cache.current_task = Task::Idle;
        self.progress_cache.percentage = 1.0;
        self.estimated_total = self.current_index;
    }
    /// Calls `finalize` and then sends the final update to the Tauri frontend.
    pub fn finalize_and_send(&mut self) {
        self.finalize();
        self.send_update();
    }
    /// Send a summary of parsed objects to the Tauri frontend.
    ///
    /// # Parameters
    ///
    /// * `summary` - A map of object types to the number of objects parsed.
    pub(crate) fn send_summary(
        &self,
        summary: &std::collections::HashMap<crate::ObjectType, usize>,
    ) {
        use tauri::Manager;

        self.tauri_window
            .emit("SUMMARY", summary.clone())
            .unwrap_or_else(|e| tracing::debug!("Tauri window emit error {:?}", e));
    }
}
