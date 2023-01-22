use pulldown_cmark::{html::push_html, Options, Parser};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{html, AttrValue, Html};

#[function_component(MarkdownComponent)]
pub fn markdown_component() -> Html {
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

    let markdown_changed = Callback::from(move |event: Event| {
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


    html! {
        <>
        <div>
            <form>
                <div class="mb-3">
                    <textarea rows="10" cols="50" class="form-control" onchange={markdown_changed}/>
                </div>
            </form>
         </div>

         if html_state.is_some() {
            <p>{Html::from_html_unchecked(AttrValue::from(format!("{}",html_state.as_deref().clone().unwrap_or_default())))}</p>
           }
        </>
    }
}
