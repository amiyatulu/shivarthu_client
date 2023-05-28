use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, AttrValue, Html};

use crate::components::markdown::markdown_to_html::parse_text_to_html;


#[function_component(MarkdownComponent)]
pub fn markdown_component() -> Html {
    let html_state: UseStateHandle<Option<String>> = use_state(|| None);
    let html_state_clone = html_state.clone();

    let markdown_changed = Callback::from(move |event: KeyboardEvent| {
        let markdown = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let html_value: Option<String> = if markdown.is_empty() {
            None
        } else {
            let html = parse_text_to_html(&markdown);
            Some(html)
        };

        html_state_clone.set(html_value);
    });

    html! {
        <>
        <div>
            <form>
                <div class="mb-3">
                    <textarea rows="10" cols="50" class="form-control" onkeyup={markdown_changed}/>
                </div>
            </form>
         </div>

         if html_state.is_some() {
            <p>{Html::from_html_unchecked(AttrValue::from(format!("{}",html_state.as_deref().clone().unwrap_or_default())))}</p>
           }
        </>
    }
}
