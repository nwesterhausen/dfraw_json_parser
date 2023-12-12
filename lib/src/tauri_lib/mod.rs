#[cfg(feature = "tauri")]
extern crate tauri;

#[cfg(feature = "tauri")]
mod progress;
#[cfg(feature = "tauri")]
mod with_progress;

#[cfg(feature = "tauri")]
pub use progress::Helper as ProgressHelper;
#[cfg(feature = "tauri")]
pub use progress::Payload as ProgressPayload;
#[cfg(feature = "tauri")]
pub use progress::Task as ProgressTask;

#[cfg(feature = "tauri")]
pub(crate) use with_progress::parse;
