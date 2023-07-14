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

#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}

#[function_component(TotalFundProfileCollected)]
pub fn total_profile_collected(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let account_id32 = AccountId32::from_str(&profile_user_account).unwrap();
    let account_id32_clone = account_id32.clone();

    let fund_needed: UseStateHandle<Option<u128>> = use_state(|| None);
    let fund_needed_clone = fund_needed.clone();


    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                    .await
                    .unwrap();
                let total_profile_fund_collected = polkadot::storage()
                    .profile_validation()
                    .profile_total_fund_collected(account_id32_clone);

                let balance = client.storage().at_latest().await.unwrap().fetch(&total_profile_fund_collected).await.unwrap().unwrap();
                fund_needed_clone.set(Some(balance));

            });
        },
        (),
    );

    html! {
        <>
        if fund_needed.is_some() {
            {"Total fund collected: "}{(*fund_needed).unwrap()}
        }
        </>
    }
}
