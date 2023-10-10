use gloo::console::log;
use subxt::{OnlineClient, PolkadotConfig};
use yew::prelude::*;

use crate::constants::constant::NODE_URL;
use crate::services::common_services::polkadot;
use std::str::FromStr;
use subxt::utils::AccountId32;
use wasm_bindgen_futures;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}

#[function_component(TotalFundProfileCollected)]
pub fn total_profile_collected(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let account_id32 = AccountId32::from_str(&profile_user_account).unwrap();
    let account_id32_clone = account_id32.clone();

    let fund_collected: UseStateHandle<Option<u128>> = use_state(|| None);
    let fund_collected_clone = fund_collected.clone();
    let registration_fee: UseStateHandle<Option<u128>> = use_state(|| None);
    let registration_fee_clone = registration_fee.clone();

    let fund_needed: UseStateHandle<Option<u128>> = use_state(|| None);
    let fund_needed_clone = fund_needed.clone();

    use_effect_with((), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                .await
                .unwrap();
            let total_profile_fund_collected = polkadot::storage()
                .profile_validation()
                .profile_total_fund_collected(account_id32_clone);

            let fund_collected_value = client
                .storage()
                .at_latest()
                .await
                .unwrap()
                .fetch_or_default(&total_profile_fund_collected)
                .await
                .unwrap();
            fund_collected_clone.set(Some(fund_collected_value));

            let registration_fee_storage =
                polkadot::storage().profile_validation().registration_fee();
            let registration_fee_value = client
                .storage()
                .at_latest()
                .await
                .unwrap()
                .fetch_or_default(&registration_fee_storage)
                .await
                .unwrap();

            // match registration_fee_result {
            //     Ok(value) => log!(value),
            //     Err(value) => log!(format!("{}",value))
            // }

            // // log!(registration_fee);
            registration_fee_clone.set(Some(registration_fee_value));

            let fund_needed = registration_fee_value.checked_sub(fund_collected_value);
            fund_needed_clone.set(fund_needed);
        });
    });

    html! {
        <>
        if fund_collected.is_some() {
            {"Total fund collected: "}{(*fund_collected).unwrap()}
        }
        <br/>
        if registration_fee.is_some() {
            {"Registration fee: "}{(*registration_fee).unwrap()}
        }
        <br/>
        if fund_needed.is_some() {
            {"Fund Needed: "}{(*fund_needed).unwrap()}
        }
        </>
    }
}
