use pulldown_cmark::{html::push_html, Options, Parser};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, AttrValue, Html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_onchange: Callback<String>, // New code: add handle_onchange props
}

#[function_component(MarkdownField)]
pub fn markdown_field(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let html_state: UseStateHandle<Option<String>> = use_state(|| None);
    let html_state_clone = html_state.clone();

    fn parse_text(value: &str) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        let parser = Parser::new_ext(&value, options);
        let mut parsed_text = String::new();
        push_html(&mut parsed_text, parser);
        parsed_text
    }

    let markdown_changed = Callback::from(move |event: KeyboardEvent| {
        let markdown = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let html_value: Option<String> = if markdown.is_empty() {
            None
        } else {
            let html = parse_text(&markdown);
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
        <div>
            <form>
                <div class="mb-3">
                    <textarea rows="10" cols="50" class="form-control" onkeyup={markdown_changed} onchange={markdown_onchange}/>
                </div>
            </form>
         </div>

         if html_state.is_some() {
            <p>{Html::from_html_unchecked(AttrValue::from(format!("{}",html_state.as_deref().clone().unwrap_or_default())))}</p>
           }
        </>
    }
}
