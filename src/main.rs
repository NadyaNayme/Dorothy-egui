#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(derive_default_enum)]
#![feature(drain_filter)]

#[cfg(not(target_arch = "wasm32"))]
use confy;
use dorothy::{app::AppDorothy, AppSettings};

fn main() -> Result<(), confy::ConfyError> {
    let cfg: AppSettings = confy::load("dorothy-egui")?;
    let app = AppDorothy::new();
    let native_options = eframe::NativeOptions {
        always_on_top: cfg.app_settings.always_on_top,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}
