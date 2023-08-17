use subxt::PolkadotConfig;
use yew::prelude::*;

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

#[function_component(GetChallengerFees)]
pub fn get_challenger_fees(props: &Props) -> Html {

    let profile_user_account = props.profile_user_account.clone();
    let account_id32 = AccountId32::from_str(&profile_user_account).unwrap();
    let account_id32_clone = account_id32.clone();

    let challenger_fee: UseStateHandle<Option<u128>> = use_state(|| None);
    let challenger_fee_clone = challenger_fee.clone();

    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                .await
                .unwrap();

            let challenger_fee_storage = polkadot::storage().profile_validation().registration_challenge_fee();

            let challenger_fee_value = client
                    .storage()
                    .at_latest()
                    .await
                    .unwrap()
                    .fetch_or_default(&challenger_fee_storage)
                    .await
                    .unwrap();

                challenger_fee_clone.set(Some(challenger_fee_value));

            });
        },
        (),
    );

    html! {
        <>
        if challenger_fee.is_some() {
            {"Challenger Fee: "}{(*challenger_fee).unwrap()}
        }
        </>
    }


}