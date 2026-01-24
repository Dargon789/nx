pub mod detection;
pub mod install;
// nx_console uses interprocess which may not work on Android
#[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
pub mod nx_console;
mod preferences;
