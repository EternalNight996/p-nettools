#![recursion_limit = "512"]
#![allow(clippy::unused_unit)]
#[macro_use]
extern crate log;
mod config;
pub mod ui;
use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    config::logger::init_wasm_log();
    info!("hello wasm");
    error!("error test");
    warn!("warn test");
    trace!("trace ? ???");
    yew::start_app::<ui::app::App>();
    Ok(())
}
