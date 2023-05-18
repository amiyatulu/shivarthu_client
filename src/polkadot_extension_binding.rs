use wasm_bindgen::prelude::*;
use js_sys::Promise;

#[wasm_bindgen(module = "/src/package.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn helloworld();

    #[wasm_bindgen]
    pub fn get_account_address_from_seed(mnemonic: String) -> String;

    #[wasm_bindgen]
    pub fn transfer_balance(wsprovider: String, mnemonic: String, credit_user: String, value: i32) -> Promise;
}