use crate::components::accounts::account_store::AccountPubStore;
use crate::components::navigation::nav::Nav;
use crate::components::profile_validation::profile::fetch_ipfs_profile::ipfs_fetch;
use crate::constants::constant::DEFAULT_IPFS_FETCH_PROVIDER;
use gloo::console::log;
use subxt::{OnlineClient, PolkadotConfig};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::constants::constant::NODE_URL;
use std::str::FromStr;
use subxt::utils::AccountId32;
use wasm_bindgen_futures;

#[subxt::subxt(
    runtime_metadata_path = "./artifacts/metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]
pub mod polkadot {}
use polkadot::runtime_types::pallet_support::Content;

#[function_component(ViewProfile)]
pub fn view_profile() -> Html {
    let (store, _) = use_store::<AccountPubStore>();
    let address_option = store.account_address.clone();
    let account_id = address_option.clone().unwrap();
    let profile_hash: UseStateHandle<Option<String>> = use_state(|| None);
    let profile_hash_clone = profile_hash.clone();
    let account_id32 = AccountId32::from_str(&account_id).unwrap();
    let account_id32_clone = account_id32.clone();
    let error_message: UseStateHandle<Option<String>> = use_state(|| None);

    let error_message_clone1 = error_message.clone();

    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                    .await
                    .unwrap();
                let citizen_id_storage = polkadot::storage()
                    .profile_validation()
                    .get_citizen_id(account_id32_clone);
                let citizen_id_option = client
                    .storage()
                    .at_latest()
                    .await
                    .unwrap()
                    .fetch(&citizen_id_storage)
                    .await
                    .unwrap();
                match citizen_id_option {
                    Some(citizen_id) => {
                        let citizen_details_storage = polkadot::storage()
                            .profile_validation()
                            .citizen_profile(citizen_id);
                        let citizen_details = client
                            .storage()
                            .at_latest()
                            .await
                            .unwrap()
                            .fetch(&citizen_details_storage)
                            .await
                            .unwrap()
                            .unwrap();
                        let content = citizen_details.content;
                        if let Content::IPFS(ipfsdata) = content {
                            let ipfs_hash = String::from_utf8(ipfsdata).unwrap();
                            log!("ipfs_hash", ipfs_hash.clone());

                            ipfs_fetch(&ipfs_hash,DEFAULT_IPFS_FETCH_PROVIDER).await;

                            profile_hash_clone.set(Some(ipfs_hash.clone()));
                        }
                    }
                    None => {
                        error_message_clone1.set(Some("Citizen details not available.".to_owned()));
                    }
                }
            })
        },
        (),
    );

    html! {
        <>
        <Nav />
        <div class="container">
        if let Some(error) = &*error_message {
            <p>{format!("{}", error)}</p>
        }
        </div>
        </>
    }
}
