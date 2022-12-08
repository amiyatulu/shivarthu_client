use gloo::console::log;
use serde::{Serialize, Deserialize};
use web_sys::File;
use gloo_net::http::{FormData, Request, Headers};
use crate::constants::auth::CRUST_TOKEN;
#[derive(Serialize, Deserialize)]
pub struct IPFSResponse {
    pub Name: String,
    pub Hash: String,
    pub Size: String,
}


pub async fn ipfs_call(file: File, name: String) -> IPFSResponse {
    let formdata = FormData::new().unwrap();
    formdata.append_with_blob(&name, &file);
    let headers = Headers::new();
    headers.append(&"Authorization", CRUST_TOKEN);


   let data =  Request::post("https://crustipfs.xyz/api/v0/add?pin=true").headers(headers).body(formdata).send().await.unwrap();
//    log!(data.text().await.unwrap());
   let body = data.json::<IPFSResponse>().await.unwrap();
   body
   // {"Name":"pexels-pixabay-326055.jpg","Hash":"QmcTJaN8SqkKLNVjWeKSVSK8zVXSodhBqxVkZZcUb1isLp","Size":"782686"}






}