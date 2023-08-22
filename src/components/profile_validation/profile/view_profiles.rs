use subxt::PolkadotConfig;
use yew::prelude::*;

use crate::constants::constant::NODE_URL;
use std::str::FromStr;
use subxt::utils::AccountId32;
use wasm_bindgen_futures;

use crate::components::profile_validation::profile::last_citizen_id::use_last_citizen_id;

#[subxt::subxt(
    runtime_metadata_path = "./artifacts/metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]

pub mod polkadot {}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}

#[function_component(ViewProfiles)]
pub fn view_profiles(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let account_id32 = AccountId32::from_str(&profile_user_account).unwrap();
    let account_id32_clone = account_id32.clone();
    let last_citizen_id = use_last_citizen_id().unwrap();

    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                    .await
                    .unwrap();

                let last_citizen_storage =
                    polkadot::storage().profile_validation().next_citizen_id();

                let last_citizen = client
                    .storage()
                    .at_latest()
                    .await
                    .unwrap()
                    .fetch_or_default(&last_citizen_storage)
                    .await
                    .unwrap();
            });
        },
        (),
    );

    html! {
        <>
        </>
    }
}
