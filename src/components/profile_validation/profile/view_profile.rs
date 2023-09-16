use crate::components::accounts::account_store::AccountPubStore;
use crate::components::markdown::markdown_to_html::parse_text_to_html;
use crate::components::navigation::nav::Nav;
use crate::components::profile_validation::profile::fetch_ipfs_profile::ipfs_fetch;
use crate::constants::constant::DEFAULT_IPFS_FETCH_PROVIDER;

use gloo::console::log;
use stylist::{yew::styled_component, Style};
use subxt::{OnlineClient, PolkadotConfig};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::constants::constant::NODE_URL;
use std::str::FromStr;
use subxt::utils::AccountId32;
use wasm_bindgen_futures;
use crate::services::common_services::polkadot;
use polkadot::runtime_types::pallet_support::Content;

const STYLE_FILE: &str = include_str!("view_profile.css");

#[styled_component(ViewProfile)]
pub fn view_profile() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();
    let (store, _) = use_store::<AccountPubStore>();
    let address_option = store.account_address.clone();
    let account_id = address_option.clone().unwrap();
    let profile_video_hash: UseStateHandle<Option<String>> = use_state(|| None);
    let profile_video_hash_clone = profile_video_hash.clone();
    let account_id32 = AccountId32::from_str(&account_id).unwrap();
    let account_id32_clone = account_id32.clone();
    let error_message: UseStateHandle<Option<String>> = use_state(|| None);
    let name_state: UseStateHandle<Option<String>> = use_state(|| None);
    let name_state_clone = name_state.clone();
    let details_state: UseStateHandle<Option<String>> = use_state(|| None);
    let details_state_clone = details_state.clone();

    // let error_message_clone1 = error_message.clone();

    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                    .await
                    .unwrap();
                let citizen_profile_storage = polkadot::storage()
                    .profile_validation()
                    .citizen_profile(account_id32_clone);
           
                let citizen_details = client
                    .storage()
                    .at_latest()
                    .await
                    .unwrap()
                    .fetch(&citizen_profile_storage)
                    .await
                    .unwrap()
                    .unwrap();
                let content = citizen_details.content;
                if let Content::IPFS(ipfsdata) = content {
                    let ipfs_hash = String::from_utf8(ipfsdata).unwrap();
                    log!("ipfs_hash", ipfs_hash.clone());

                    let resp = ipfs_fetch(&ipfs_hash, DEFAULT_IPFS_FETCH_PROVIDER).await;

                    profile_video_hash_clone.set(Some(resp.profile_video_cid.clone()));
                    name_state_clone.set(Some(resp.name.clone()));
                    details_state_clone.set(Some(resp.details.clone()));
                }
            })
        },
        (),
    );

    html! {
        <>
        <Nav />
        <div class={classes!("container", stylesheet)}>
        if let Some(error) = &*error_message {
            <div class="mb-3">
                <h2 class="heading">{"Error"} </h2>
                <p class="data">{format!("{}", error)}</p>
            </div>
        }
        if let Some(name) = &*name_state {
            <div class="mb-3">
                <h2 class="heading">{"Name"} </h2>
                <p class="data">{format!("{}", name)}</p>
            </div>
        }
        if let Some(details) = &*details_state {
            <div class="mb-3">
                <h2 class="heading">{"Details"} </h2>
                <div class="data">{Html::from_html_unchecked(AttrValue::from(parse_text_to_html(&format!("{}", details))))}</div>
            </div>
        }

        if profile_video_hash.is_some() {
            <div class="mb-3">
                <video width="320" height="240" controls={true}>
                <source src={format!("{}{}", DEFAULT_IPFS_FETCH_PROVIDER.address, profile_video_hash.as_deref().unwrap_or_default())} type="video/mp4" />
                {"Your browser does not support the video tag."}
                </video>
            </div>
        }
        </div>
        </>
    }
}
