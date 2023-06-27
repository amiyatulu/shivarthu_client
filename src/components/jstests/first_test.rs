use crate::components::accounts::hooks::balance_transfer_hook;
use crate::constants::auth::HF_ACCESS_TOKEN;
use crate::constants::constant::{HUGGING_FACE_MODEL, NODE_URL};
use crate::js_extension_binding;
use gloo::console::log;
use gloo_timers::callback::Timeout;
use wasm_bindgen_futures;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;
use regex::Regex;


#[function_component(FirstTest)]
pub fn storage() -> Html {
    let hookdata = balance_transfer_hook::use_balance_tranfer(
        "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y".to_owned(),
        10000,
    );
    let chat_text = use_state(|| "".to_owned());
    let chat_text_clone = chat_text.clone();
    let hookdata_clone2 = hookdata.clone();
    let re = Regex::new(r"\n").unwrap();


    use_effect_with_deps(
        move |_| {
            let hookdata_clone = hookdata.clone();
            let timeout = Timeout::new(1_000, move || {
                wasm_bindgen_futures::spawn_local(async move {
                    // polkadot_extension_binding::helloworld();
                    // let data = polkadot_extension_binding::get_account_address_from_seed("//Alice".to_owned());
                    // log!(data);
                    let prompt = "Explain ownership in Rust programming?".to_owned();
                    let chatpromisedata = js_extension_binding::chat_text(
                        HF_ACCESS_TOKEN.to_owned(),
                        prompt,
                        HUGGING_FACE_MODEL.to_owned()
                    );
                    
                    let chatjsresult = JsFuture::from(chatpromisedata).await.unwrap();
                    let re = re.clone();
                    let chatdataresult = format!("{}", chatjsresult.clone().as_string().unwrap());
                    // log!();
                    let chatdata = re.replace_all(&chatdataresult, "<br/>");
                    log!(format!("{}", chatdata.clone()));


                    chat_text_clone.set(format!("{}", chatdata.clone()));

                    let promisedata = js_extension_binding::add_profile(
                        NODE_URL.to_owned(),
                        "//Alice".to_owned(),
                        "bafykbzacecb53zxvj6q6q63rintnpcuhufbojz56q3zg4xxpzmzgmi2pplnmi".to_owned(),
                    );
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
        <>
        <div class="container">
            <h1>{"Storage"}</h1>
            <p>{Html::from_html_unchecked(AttrValue::from(format!("<div>{}</div>",*chat_text)))}</p>
        </div>
        </>
    }
}
