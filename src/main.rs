#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_arch = "wasm32"))]

fn main() {
    let app = dorothy::Dorothy::new();
    let mut native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
