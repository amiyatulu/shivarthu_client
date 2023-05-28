use crate::constants::constant::IPFSFetchProvider;
use gloo::console::log;
use gloo_net::http::{FormData, Headers, Request};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProfileFetchResponse {
    pub version: String,
    pub name: String,
    pub details: String,
    pub profile_video_cid: String,
}

pub async fn ipfs_fetch(
    hash: &str,
    ipfs_fetch_provider: IPFSFetchProvider<'_>,
) -> ProfileFetchResponse {
    let resp = Request::get(&format!("{}{}", ipfs_fetch_provider.address, hash))
        .send()
        .await
        .unwrap();
    let body = resp.json::<ProfileFetchResponse>().await.unwrap();
    // log!(body.name);
    body
}
