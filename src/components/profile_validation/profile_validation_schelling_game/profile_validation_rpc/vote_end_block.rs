use gloo::console::log;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use wasm_bindgen_futures;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}

#[function_component(VoteEndBlock)]
pub fn vote_end_block(props: &Props) -> Html {
   let profile_user_account = props.profile_user_account.clone();
   let end_period: UseStateHandle<Option<u32>> = use_state(|| None);
   let end_period_clone = end_period.clone();
   let end_period_clone2 = end_period.clone();



    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let client = WasmClientBuilder::default()
                    .build("ws://127.0.0.1:9944")
                    .await
                    .unwrap();
                let result: Option<u32> = client
                    .request(
                        "profilevalidation_voteendblock",
                        rpc_params![profile_user_account],
                    )
                    .await
                    .unwrap();

                // log!(result);

                end_period_clone.set(result);

                // log!(serde_json::to_string_pretty(&period).unwrap());
            });
        },
        (),
    );

    html! {
        <>
        <h1>{"RPC"}</h1>
        if end_period_clone2.is_some() {
            {*end_period}
        } else {
            {"None"}
        }
        </>
    }
}
