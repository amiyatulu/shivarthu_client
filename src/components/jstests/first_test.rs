use crate::components::accounts::hooks::balance_transfer_hook;
use crate::polkadot_extension_binding;
use gloo::console::log;
use gloo_timers::callback::Timeout;
use wasm_bindgen_futures;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;
use crate::constants::constant::NODE_URL;

#[function_component(FirstTest)]
pub fn storage() -> Html {
    let hookdata = balance_transfer_hook::use_balance_tranfer(
        "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y".to_owned(),
        10000,
    );

    let hookdata_clone2 = hookdata.clone();

    use_effect_with_deps(
        move |_| {
            let hookdata_clone = hookdata.clone();
            let timeout = Timeout::new(1_000, move || {
                wasm_bindgen_futures::spawn_local(async move {
                    // polkadot_extension_binding::helloworld();
                    // let data = polkadot_extension_binding::get_account_address_from_seed("//Alice".to_owned());
                    // log!(data);

                    let promisedata = polkadot_extension_binding::add_profile(NODE_URL.to_owned(), "//Alice".to_owned(), "bafykbzacecb53zxvj6q6q63rintnpcuhufbojz56q3zg4xxpzmzgmi2pplnmi".to_owned());
                    let js_result = JsFuture::from(promisedata).await.unwrap();
                    log!("data promise", js_result.as_string());
                    // log!("hookdata", hookdata_clone.value);
                });
            });
            timeout.forget();

            || {}
        },
        hookdata_clone2,
    );

    html! {
        <h1>{"Storage"}</h1>
    }
}
