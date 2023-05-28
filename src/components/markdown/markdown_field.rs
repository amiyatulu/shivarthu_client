use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, AttrValue, Html};
use crate::components::markdown::markdown_to_html::parse_text_to_html;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub class: String,
    pub name: String,
    pub required: bool,
    pub handle_onchange: Callback<String>, // New code: add handle_onchange props
}

#[function_component(MarkdownField)]
pub fn markdown_field(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let required = props.required.clone();
    let class = props.class.clone();
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

    let markdown_onchange = Callback::from(move |event: Event| {
        let markdown = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        handle_onchange.emit(markdown);
    });

    html! {
        <>  
              
       <textarea rows="10" cols="50" name={props.name.clone()} class={class} onkeyup={markdown_changed} onchange={markdown_onchange} required={required}/>
                


         if html_state.is_some() {
            <p>{Html::from_html_unchecked(AttrValue::from(format!("{}",html_state.as_deref().clone().unwrap_or_default())))}</p>
           }
        </>
    }
}
