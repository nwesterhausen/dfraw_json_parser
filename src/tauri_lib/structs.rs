#[cfg(feature = "tauri")]
#[derive(Clone, serde::Serialize)]
/// It's a struct to represent the progress of the current job. This is emitted back to the Tauri app using the `PROGRESS` event.
///
/// Properties:
///
/// * `percentage`: The percentage of completed steps out of total steps.
/// * `current_module`: The name of the module that is currently being processed.
pub struct ProgressPayload {
    #[serde(rename = "currentTask")]
    current_task: String,
    percentage: f64,
    #[serde(rename = "currentModule")]
    current_module: String,
    #[serde(rename = "currentFile")]
    current_file: String,
    #[serde(rename = "currentLocation")]
    current_location: String,
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
    #[allow(clippy::cast_precision_loss)]
    fn step_advance(&mut self) {
        self.current_step += 1;
        self.progress_cache.percentage = self.current_step as f64 / self.total_steps as f64;
    }
    pub fn send_update(&mut self, current_file: &str) {
        self.step_advance();
        self.progress_cache.current_file = String::from(current_file);
        if let Err(e) = self.tauri_window.emit("PROGRESS", &self.progress_cache) {
            log::debug!("Tauri window emit error {:?}", e);
        };
    }
    pub fn send_final(&mut self, message: &str) {
        self.progress_cache.current_file = String::from("None");
        self.progress_cache.current_module = String::from("None");
        self.progress_cache.current_task = String::from(message);
        self.progress_cache.percentage = 1.0;
        if let Err(e) = self.tauri_window.emit("PROGRESS", &self.progress_cache) {
            log::debug!("Tauri window emit error {:?}", e);
        };
    }
}
