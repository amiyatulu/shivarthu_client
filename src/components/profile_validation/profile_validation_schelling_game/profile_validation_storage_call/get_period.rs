use subxt::{OnlineClient, PolkadotConfig};
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
use polkadot::runtime_types::schelling_game_shared::types::Period;
use polkadot::runtime_types::sortition_sum_game::types::SumTreeName;

#[function_component(GetPeriod)]
pub fn get_period(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let account_id32 = AccountId32::from_str(&profile_user_account).unwrap();
    let account_id32_clone = account_id32.clone();
    let account_id32_clone2 = account_id32.clone();

    let block_number: UseStateHandle<Option<u32>> = use_state(|| None);
    let block_number_clone = block_number.clone();
    let period_name: UseStateHandle<Option<Period>> = use_state(|| None);
    let period_name_clone = period_name.clone();

    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                    .await
                    .unwrap();
                let profile_validation_block_storage = polkadot::storage()
                    .profile_validation()
                    .profile_validation_block(account_id32_clone);

                let profile_validation_block = client
                    .storage()
                    .at_latest()
                    .await
                    .unwrap()
                    .fetch(&profile_validation_block_storage)
                    .await
                    .unwrap();
                block_number_clone.set(profile_validation_block);

                if profile_validation_block.is_some() {

                    let key = SumTreeName::ProfileValidation {
                        citizen_address: account_id32_clone2,
                        block_number: profile_validation_block.unwrap(),
                    };
    
                    let period_storage = polkadot::storage().schelling_game_shared().period_name(key);
                    let period = client
                        .storage()
                        .at_latest()
                        .await
                        .unwrap()
                        .fetch(&period_storage)
                        .await
                        .unwrap();
                    period_name_clone.set(period);

                };

                
            });
        },
        (),
    );

    html! {
        <>
        if period_name.is_some() {
            {"Period Name: "}{format!("{:?}", period_name.as_ref().unwrap())}
        }
        <br/>

        </>
    }
}
