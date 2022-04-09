#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(derive_default_enum)]
#![feature(drain_filter)]

#[cfg(not(target_arch = "wasm32"))]
use dorothy_egui::app::AppDorothy;
use dorothy_egui::DOROTHY;
use eframe::epi::IconData;

fn main() {
    let dorothy_icon: Vec<u8> = image::load_from_memory(DOROTHY)
        .unwrap()
        .into_rgba8()
        .to_vec();
    let app_icon: IconData = IconData {
        rgba: dorothy_icon,
        width: 32,
        height: 32,
    };

    let app = AppDorothy::default();
    let native_options = eframe::NativeOptions {
        icon_data: Some(app_icon),
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}
