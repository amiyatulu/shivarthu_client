use gloo::console::log;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use wasm_bindgen_futures;
use yew::prelude::*;
use yew_hooks::prelude::*;
use crate::constants::constant::NODE_URL;




#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}

#[function_component(StakingEndBlock)]
pub fn staking_end_block(props: &Props) -> Html {
   let profile_user_account = props.profile_user_account.clone();
   let end_period: UseStateHandle<Option<u32>> = use_state(|| None);
   let end_period_clone = end_period.clone();
   let end_period_clone2 = end_period.clone();



   use_interval(
        move || {
            let end_period_clone = end_period_clone.clone();
            let profile_user_account = profile_user_account.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let client = WasmClientBuilder::default()
                    .build(NODE_URL)
                    .await
                    .unwrap();
                let result: Option<u32> = client
                    .request(
                        "profilevalidation_stakingperiodendblock",
                        rpc_params![profile_user_account],
                    )
                    .await
                    .unwrap();

                // log!(result);

                end_period_clone.set(result);

                // log!(serde_json::to_string_pretty(&period).unwrap());
            });
        },
        10000,
    );

    html! {
        <>
       <p>
        if end_period_clone2.is_some() {
            {"Staking Period ends: "}{*end_period}
        } else {
            {"Staking Period ends: "}{"None"}
        }
        </p>
        </>
    }
}
