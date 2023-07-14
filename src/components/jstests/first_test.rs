use crate::components::accounts::hooks::balance_transfer_hook;
use crate::constants::auth::HF_ACCESS_TOKEN;
use crate::constants::constant::{HUGGING_FACE_MODEL, NODE_URL};
use crate::js_extension_binding;
use gloo::console::log;
use gloo_timers::callback::Timeout;
use regex::Regex;
use wasm_bindgen_futures;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;

#[function_component(FirstTest)]
pub fn storage() -> Html {
    let chat_text = use_state(|| "".to_owned());
    let chat_text_clone = chat_text.clone();

    use_effect_with_deps(
        move |_| {
            let timeout = Timeout::new(1_000, move || {
                wasm_bindgen_futures::spawn_local(async move {
                    let promisedata = js_extension_binding::add_profile_stake(
                        NODE_URL.to_owned(),
                        "//Alice".to_owned(),
                        "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY".to_owned(),
                        500,
                    );
                    let js_result = JsFuture::from(promisedata).await.unwrap();
                    log!("data promise", js_result.as_string());

                    // let promisedata = js_extension_binding::add_profile(
                    //     NODE_URL.to_owned(),
                    //     "//Alice".to_owned(),
                    //     "bafykbzacecb53zxvj6q6q63rintnpcuhufbojz56q3zg4xxpzmzgmi2pplnmi".to_owned(),
                    // );
                    // let js_result = JsFuture::from(promisedata).await.unwrap();
                    // log!("data promise", js_result.as_string());
                    // log!("hookdata", hookdata_clone.value);
                });
            });
            timeout.forget();

            || {}
        },
        (),
    );

    html! {
        <>
        <div class="container">
            <h1>{"Storage"}</h1>
        </div>
        </>
    }
}
