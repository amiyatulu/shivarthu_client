use gloo::console::log;
use gloo_net::http::{FormData, Headers, Request};
use crate::constants::constant::IPFSFetchProvider;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProfileFetchResponse {
    pub version: String,
    pub name: String,
    pub details: String,
    pub profile_video_cid: String,
}

pub async fn ipfs_fetch(hash: &str, ipfs_fetch_provider: IPFSFetchProvider<'_>) {
    ipfs_res(hash, &ipfs_fetch_provider.address ).await;
}

pub async fn ipfs_res(hash: &str, url: &str) {
    let resp = Request::get(&format!("{}{}", url, hash)).send().await.unwrap();
    let body = resp.json::<ProfileFetchResponse>().await.unwrap();
    log!(body.name)
}