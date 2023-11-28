#[cfg(feature = "tauri")]
extern crate tauri;

#[cfg(feature = "tauri")]
mod structs;
#[cfg(feature = "tauri")]
mod with_progress;

#[cfg(feature = "tauri")]
pub use structs::ProgressHelper;
#[cfg(feature = "tauri")]
pub use structs::ProgressPayload;
#[cfg(feature = "tauri")]
pub(crate) use with_progress::parse;
