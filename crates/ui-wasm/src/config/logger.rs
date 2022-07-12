use wasm_logger;
pub fn init_wasm_log() {
    wasm_logger::init(wasm_logger::Config::default());
}