#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::rand_api;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::rand_api;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::rand_api;

mod fallback;
pub use fallback::*;