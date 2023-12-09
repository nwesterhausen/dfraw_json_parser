use tracing::debug;

#[derive(Default, Clone, serde::Serialize)]
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
}

#[cfg(feature = "tauri")]
pub struct ProgressHelper {
    total_steps: usize,
    current_step: usize,
    tauri_window: tauri::Window,
    progress_cache: ProgressPayload,
}

#[cfg(feature = "tauri")]
impl ProgressHelper {
    pub fn with_tauri_window(window: tauri::Window) -> Self {
        Self {
            total_steps: 1,
            current_step: 0,
            tauri_window: window,
            progress_cache: ProgressPayload {
                current_task: String::new(),
                percentage: 0.0,
                current_module: String::new(),
                current_file: String::new(),
                current_location: String::new(),
                running_total: 0,
            },
        }
    }
    pub fn add_steps(&mut self, amount: usize) {
        self.total_steps += amount;
    }
    pub fn update_current_location(&mut self, location: &str) {
        self.progress_cache.current_location = String::from(location);
    }
    pub fn update_current_module(&mut self, module: &str) {
        self.progress_cache.current_module = String::from(module);
    }
    pub fn update_current_task(&mut self, task: &str) {
        self.progress_cache.current_task = String::from(task);
    }
    pub fn add_to_running_total(&mut self, amount: usize) {
        self.progress_cache.running_total += amount;
    }
    #[allow(clippy::cast_precision_loss)]
    fn step_advance(&mut self) {
        self.current_step += 1;
        self.progress_cache.percentage = self.current_step as f64 / self.total_steps as f64;
    }
    pub fn send_update(&mut self, current_file: &str) {
        use tauri::Manager;

        self.step_advance();
        self.progress_cache.current_file = String::from(current_file);
        if let Err(e) = self.tauri_window.emit("PROGRESS", &self.progress_cache) {
            debug!("Tauri window emit error {:?}", e);
        };
    }
    pub fn send_final(&mut self, message: &str) {
        use tauri::Manager;

        self.progress_cache.current_file = String::from("None");
        self.progress_cache.current_module = String::from("None");
        self.progress_cache.current_task = String::from(message);
        self.progress_cache.percentage = 1.0;
        if let Err(e) = self.tauri_window.emit("PROGRESS", &self.progress_cache) {
            debug!("Tauri window emit error {:?}", e);
        };
    }

    pub(crate) fn send_summary(
        &self,
        summary: &std::collections::HashMap<crate::ObjectType, usize>,
    ) {
        use tauri::Manager;

        self.tauri_window
            .emit("SUMMARY", summary.clone())
            .unwrap_or_else(|e| debug!("Tauri window emit error {:?}", e));
    }
}
