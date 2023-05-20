use wasm_bindgen::prelude::*;
use js_sys::Promise;

#[wasm_bindgen(module = "/src/package.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn helloworld();

    #[wasm_bindgen]
    pub fn get_account_address_from_seed(mnemonic: String) -> String;

    #[wasm_bindgen]
    pub fn transfer_balance(wsprovider: String, mnemonic: String, credit_user: String, value: u32) -> Promise;

    #[wasm_bindgen]
    pub fn add_profile(wsprovider: String, mnemonic: String, ipfs_string: String) -> Promise;
   
    // bafykbzacecb53zxvj6q6q63rintnpcuhufbojz56q3zg4xxpzmzgmi2pplnmi
    
}