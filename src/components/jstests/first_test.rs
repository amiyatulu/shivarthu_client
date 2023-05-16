use crate::polkadot_extension_binding;
use gloo::console::log;
use gloo_timers::callback::Timeout;
use wasm_bindgen_futures;
use yew::prelude::*;

#[function_component(FirstTest)]
pub fn storage() -> Html {
    let first_load = use_state(|| true);

    use_effect(move || {
        let timeout = Timeout::new(1_000, move || {
            wasm_bindgen_futures::spawn_local(async move {
                polkadot_extension_binding::helloworld();
            });
        });
        timeout.forget();
        if *first_load {
            //code
            first_load.set(false);
        }
        || {}
    });

    html! {
        <h1>{"Storage"}</h1>
    }
}
