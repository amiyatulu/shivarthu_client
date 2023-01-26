use crate::components::api::select_ipfs_provider::IPFSProvider;
use crate::components::api::select_ipfs_provider::{CRUST_GATEWAY, WEB3_STORAGE_API_UPLOAD};
use crate::constants::auth::{CRUST_TOKEN, WEB3_STORAGE_TOKEN};


use gloo::console::log;
use gloo_net::http::{FormData, Headers, Request};
use serde::{Deserialize, Serialize};
use web_sys::{Blob, File};
use wasm_bindgen::JsValue;
#[derive(Serialize, Deserialize)]
pub struct IPFSResponse {
    pub Name: String,
    pub Hash: String,
    pub Size: String,
}

#[derive(Serialize, Deserialize)]
pub struct WEB3StorageResponse {
    pub cid: String,
}

pub async fn ipfs_call(ipfs_provider: IPFSProvider, file: File, name: String) -> String {
    match ipfs_provider {
        IPFSProvider::Crust => ipfs_call_crust(file, name).await,
        IPFSProvider::Web3Storage => ipfs_call_web3storage(file, name).await,
    }
}

pub async fn ipfs_call_json_string(
    ipfs_provider: IPFSProvider,
    data: &str,
    name: String,
) -> String {
    match ipfs_provider {
        IPFSProvider::Crust => ipfs_call_json_string_crust(data, name).await,
        IPFSProvider::Web3Storage => ipfs_call_json_string_web3storage(data, name).await,
    }
}

pub async fn ipfs_call_crust(file: File, name: String) -> String {
    let formdata = FormData::new().unwrap();
    formdata.append_with_blob(&name, &file);
    let headers = Headers::new();
    headers.append(&"Authorization", CRUST_TOKEN);

    let data = Request::post(&format!("{CRUST_GATEWAY}/api/v0/add?pin=true"))
        .headers(headers)
        .body(formdata)
        .send()
        .await
        .unwrap();
    //    log!(data.text().await.unwrap());
    let body = data.json::<IPFSResponse>().await.unwrap();
    body.Hash
    // {"Name":"pexels-pixabay-326055.jpg","Hash":"QmcTJaN8SqkKLNVjWeKSVSK8zVXSodhBqxVkZZcUb1isLp","Size":"782686"}
}

pub async fn ipfs_call_json_string_crust(data: &str, name: String) -> String {
    let formdata = FormData::new().unwrap();
    formdata.append_with_str(&name, &data);
    let headers = Headers::new();
    headers.append(&"Authorization", CRUST_TOKEN);

    let data = Request::post(&format!("{CRUST_GATEWAY}/api/v0/add?pin=true"))
        .headers(headers)
        .body(formdata)
        .send()
        .await
        .unwrap();
    //    log!(data.text().await.unwrap());
    let body = data.json::<IPFSResponse>().await.unwrap();
    body.Hash
    // {"Name":"pexels-pixabay-326055.jpg","Hash":"QmcTJaN8SqkKLNVjWeKSVSK8zVXSodhBqxVkZZcUb1isLp","Size":"782686"}
}

pub async fn ipfs_call_web3storage(file: File, _name: String) -> String {
    let headers = Headers::new();
    headers.append(&"accept", "application/octet-stream");
    headers.append(&"Authorization", WEB3_STORAGE_TOKEN);

    let data = Request::post(WEB3_STORAGE_API_UPLOAD)
        .headers(headers)
        .body(file)
        .send()
        .await
        .unwrap();
    //    log!(data.text().await.unwrap());
    let body = data.json::<WEB3StorageResponse>().await.unwrap();
    body.cid
    // {"Name":"pexels-pixabay-326055.jpg","Hash":"QmcTJaN8SqkKLNVjWeKSVSK8zVXSodhBqxVkZZcUb1isLp","Size":"782686"}
}

pub async fn ipfs_call_json_string_web3storage(data: &str, _name: String) -> String {
    let json_jsvalue = JsValue::from_str(&data);
    let json_jsvalue_array = js_sys::Array::from_iter(std::iter::once(json_jsvalue));
    let json_blob_result = Blob::new_with_str_sequence(&json_jsvalue_array);
    let json_blob = json_blob_result.unwrap();
    let headers = Headers::new();
    headers.append(&"accept", "application/octet-stream");
    headers.append(&"Authorization", WEB3_STORAGE_TOKEN);

    let data = Request::post(WEB3_STORAGE_API_UPLOAD)
        .headers(headers)
        .body(json_blob)
        .send()
        .await
        .unwrap();
    // log!(data.text().await.unwrap());

    let body = data.json::<WEB3StorageResponse>().await.unwrap();
    body.cid
}
