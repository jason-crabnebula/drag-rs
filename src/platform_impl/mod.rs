#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod platform;
#[cfg(target_os = "linux")]
#[cfg(feature = "x11")]
#[path = "x11/mod.rs"]
mod platform;
#[cfg(target_os = "linux")]
#[cfg(feature = "gtk")]
#[path = "gtk/mod.rs"]
mod platform;
#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
mod platform;

// pub use platform::start_drag;
pub use platform::*;
