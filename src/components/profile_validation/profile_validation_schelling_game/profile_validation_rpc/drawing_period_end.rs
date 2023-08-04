use gloo::console::log;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use wasm_bindgen_futures;
use yew::prelude::*;
use std::ops::Deref;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}

#[function_component(DrawingPeriodEnd)]
pub fn drawing_period_end(props: &Props) -> Html {
   let profile_user_account = props.profile_user_account.clone();
   let drawing_period_value: UseStateHandle<Option<(u64, u64, bool)>> = use_state(|| None);
   let drawing_period_value_clone = drawing_period_value.clone();
   let drawing_period_value_clone2 = drawing_period_value.clone();



    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let client = WasmClientBuilder::default()
                    .build("ws://127.0.0.1:9944")
                    .await
                    .unwrap();
                let result: (u64, u64, bool) = client
                    .request(
                        "profilevalidation_drawingperiodend",
                        rpc_params![profile_user_account],
                    )
                    .await
                    .unwrap();

                // log!(result);

                drawing_period_value_clone.set(Some(result));

                // log!(serde_json::to_string_pretty(&period).unwrap());
            });
        },
        (),
    );

    html! {
        <>
        <h1>{"RPC"}</h1>
        if drawing_period_value_clone2.is_some() {
            {drawing_period_value.deref().unwrap().0}
        } else {
            {"None"}
        }
        </>
    }
}
