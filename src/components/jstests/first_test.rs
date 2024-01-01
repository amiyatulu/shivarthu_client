use crate::components::accounts::hooks::balance_transfer_hook;
use crate::constants::auth::EVERLAND_CREDENTIALS;
use crate::constants::auth::HF_ACCESS_TOKEN;
use crate::constants::constant::{EVERLAND_ENDPOINT, HUGGING_FACE_MODEL, NODE_URL};
use crate::js_extension_binding;
use gloo::console::log;
use gloo_timers::callback::Timeout;
// use ipfs_hasher::IpfsHasher;
use regex::Regex;
use wasm_bindgen_futures;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;

#[function_component(FirstTest)]
pub fn storage() -> Html {
    let chat_text = use_state(|| "".to_owned());
    let chat_text_clone = chat_text.clone();

    use_effect_with((), move |_| {
        let timeout = Timeout::new(1_000, move || {
            wasm_bindgen_futures::spawn_local(async move {
                let endpoint = EVERLAND_ENDPOINT.to_owned();
                let bucket = "shivarthu-upload".to_owned();
                let folder = "website_tests".to_owned();
                let data = "test through client".to_owned();
                let file_name = "test_file.txt".to_owned();
                let key = EVERLAND_CREDENTIALS.0.to_owned();
                let secret = EVERLAND_CREDENTIALS.1.to_owned();
                let session_token = EVERLAND_CREDENTIALS.2.to_owned();

                // let hasher = IpfsHasher::default();
                // let cid = hasher.compute(data.clone().as_bytes());
                // log!("cid:", cid);

                let promisedata = js_extension_binding::upload_file_4everland(
                    endpoint,
                    bucket,
                    folder,
                    data,
                    file_name,
                    key,
                    secret,
                    session_token,
                );
                let js_result = JsFuture::from(promisedata).await.unwrap();
                log!("data promise", js_result.as_string());
                // let promisedata = js_extension_binding::add_profile_stake(
                //     NODE_URL.to_owned(),
                //     "//Alice".to_owned(),
                //     "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY".to_owned(),
                //     500,
                // );
                // let js_result = JsFuture::from(promisedata).await.unwrap();
                // log!("data promise", js_result.as_string());

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
    });

    html! {
        <>
        <div class="container">
            <h1>{"Storage"}</h1>
        </div>
        </>
    }
}
