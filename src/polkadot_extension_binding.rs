use wasm_bindgen::prelude::*;
use js_sys::Promise;

#[wasm_bindgen(module = "/src/package.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn helloworld();

    #[wasm_bindgen]
    pub fn get_account_address_from_seed(mnemonic: String) -> String;


}