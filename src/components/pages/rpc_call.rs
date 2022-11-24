use gloo::console::log;
use wasm_bindgen_futures;
use yew::prelude::*;
use jsonrpsee_core::{
	client::{ClientT},
	rpc_params,
};
use jsonrpsee_wasm_client::WasmClientBuilder;


#[function_component(Rpc)]
pub fn rpc() -> Html {
    let first_load = use_state(|| true);

    use_effect(move || {
        if *first_load {
            wasm_bindgen_futures::spawn_local(async move {
            
                let client =
                WasmClientBuilder::default().build("ws://127.0.0.1:9944")
                        .await
                        .unwrap();
                let result: u128 = client.request("shivarthu_helloWorld", rpc_params![]).await.unwrap();

                log!(result);
        
                // log!(serde_json::to_string_pretty(&period).unwrap());
            });
            first_load.set(false);
        }
        || {}
    });

    html! {
        <h1>{"RPC"}</h1>
    }
}
