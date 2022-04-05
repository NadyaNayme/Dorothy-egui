#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::Dorothy;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let app = Dorothy::default();
    eframe::start_web(canvas_id, Box::new(app))
}
