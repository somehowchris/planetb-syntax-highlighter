use wasm_bindgen::prelude::*;

// wasm-bindgen will automatically take care of including this script
#[wasm_bindgen(module = "/src/highlighter/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = "highlight")]
    pub fn highlight() -> u8;
}