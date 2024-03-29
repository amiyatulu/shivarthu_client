use gloo::console::log;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use wasm_bindgen_futures;
use yew::prelude::*;
use std::ops::Deref;
use yew_icons::{Icon, IconId};

use crate::constants::constant::NODE_URL;




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



   use_effect_with((),
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let client = WasmClientBuilder::default()
                    .build(NODE_URL)
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
    );

    html! {
        <>
        <p>
        if drawing_period_value_clone2.is_some() {
            {"Drawing Period ends: "} <span id="end-drawing-period-time">{drawing_period_value.deref().unwrap().2}</span>
        } else {
            {"Drawing Period ends: "} <span id="end-drawing-period-time"><Icon icon_id={IconId::FontAwesomeSolidSpinner} /></span>
        }
        </p>
        </>
    }
}
