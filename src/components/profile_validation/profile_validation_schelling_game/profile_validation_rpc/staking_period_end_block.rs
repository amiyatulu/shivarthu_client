use gloo::console::log;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use wasm_bindgen_futures;
use yew::prelude::*;

#[function_component(StakingPeriodEndBlock)]
pub fn staking_period_end_block() -> Html {
   let end_period: UseStateHandle<Option<u32>> = use_state(|| None);
   let end_period_clone = end_period.clone();


    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let client = WasmClientBuilder::default()
                    .build("ws://127.0.0.1:9944")
                    .await
                    .unwrap();
                let result: Option<u32> = client
                    .request(
                        "profilevalidation_stakingperiodendblock",
                        rpc_params!["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],
                    )
                    .await
                    .unwrap();

                log!(result);

                // log!(serde_json::to_string_pretty(&period).unwrap());
            });
        },
        (),
    );

    html! {
        <>
        <h1>{"RPC"}</h1>
        if end_period_clone.is_some() {
            {*end_period}
        } else {
            {"None"}
        }
        </>
    }
}
