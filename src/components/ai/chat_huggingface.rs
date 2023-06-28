use crate::constants::auth::HF_ACCESS_TOKEN;
use crate::constants::constant::{HUGGING_FACE_MODEL};
use crate::js_extension_binding;
use gloo::console::log;
use gloo_timers::callback::Timeout;
use regex::Regex;
use wasm_bindgen_futures;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;

#[function_component(ChatHuggingFace)]
pub fn storage() -> Html {
    let chat_text = use_state(|| "".to_owned());
    let chat_text_clone = chat_text.clone();
    let re = Regex::new(r"\n").unwrap();

    use_effect_with_deps(
        move |_| {
            let timeout = Timeout::new(1_000, move || {
                wasm_bindgen_futures::spawn_local(async move {
                    let prompt = "What are some of the most common features and best practices of Rust programming language?".to_owned();
                    let chatpromisedata = js_extension_binding::chat_text(
                        HF_ACCESS_TOKEN.to_owned(),
                        prompt,
                        HUGGING_FACE_MODEL.to_owned(),
                    );

                    let chatjsresult = JsFuture::from(chatpromisedata).await.unwrap();
                    let re = re.clone();
                    let chatdataresult = format!("{}", chatjsresult.clone().as_string().unwrap());
   
                    let chatdata = re.replace_all(&chatdataresult, "<br/>");
                    // log!(format!("{}", chatdata.clone()));

                    chat_text_clone.set(format!("{}", chatdata.clone()));
                 
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
            <h1>{"Chat"}</h1>
            <p>{Html::from_html_unchecked(AttrValue::from(format!("<div>{}</div>",*chat_text)))}</p>
        </div>
        </>
    }
}
