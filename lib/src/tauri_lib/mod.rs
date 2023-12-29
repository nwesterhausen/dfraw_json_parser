/// Progress helpers for the Tauri emitter. These enable the Tauri frontend to display progress information.
#[cfg(feature = "tauri")]
mod progress;
/// A modified version of the `parse` function that uses the Tauri emitter.
#[cfg(feature = "tauri")]
mod with_progress;

#[cfg(feature = "tauri")]
pub use progress::Helper as ProgressHelper;
#[cfg(feature = "tauri")]
pub use progress::Payload as ProgressPayload;
#[cfg(feature = "tauri")]
pub use progress::Task as ProgressTask;

/// A modified version of the `parse` function that uses the Tauri emitter.
#[cfg(feature = "tauri")]
pub(crate) use with_progress::parse;
