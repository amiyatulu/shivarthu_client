use gloo::console::log;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use wasm_bindgen_futures;
use yew::prelude::*;
use std::ops::Deref;
use crate::constants::constant::NODE_URL;



#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}

#[function_component(SelectedAsJuror)]
pub fn selected_as_juror(props: &Props) -> Html {
   let profile_user_account = props.profile_user_account.clone();
   let juror_selected: UseStateHandle<Option<bool>> = use_state(|| None);
   let juror_selected_clone = juror_selected.clone();
   let juror_selected_clone2 = juror_selected.clone();



    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let client = WasmClientBuilder::default()
                    .build(NODE_URL)
                    .await
                    .unwrap();
                let result: bool = client
                    .request(
                        "profilevalidation_selectedjuror",
                        rpc_params![profile_user_account],
                    )
                    .await
                    .unwrap();

                // log!(result);

                juror_selected_clone.set(Some(result));

                // log!(serde_json::to_string_pretty(&period).unwrap());
            });
        },
        (),
    );

    html! {
        <>
        <h1>{"RPC"}</h1>
        if juror_selected_clone2.is_some() {
            {juror_selected.deref().unwrap()}
        } else {
            {"None"}
        }
        </>
    }
}
