use wasm_bindgen::prelude::*;
use js_sys::Promise;

#[wasm_bindgen(module = "/src/package.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn helloworld();


}