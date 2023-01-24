use crate::components::markdown::markdown_field::MarkdownField;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(AddProfile)]
pub fn add_profile() -> Html {
    let name_state: UseStateHandle<Option<String>> = use_state(|| Some("".to_owned()));
    let markdown_state: UseStateHandle<Option<String>> = use_state(|| Some("".to_owned()));
    let name_state_clone = name_state.clone();
    let markdown_state_clone = markdown_state.clone();

    let name_changed = Callback::from(move |event: Event| {
        let name = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let name_value: Option<String> = if name.is_empty() { None } else { Some(name) };

        name_state_clone.set(name_value);
    });

    let markdown_changed = Callback::from(move |markdown: String| {
        let markdown_value: Option<String> = if markdown.is_empty() {
            None
        } else {
            Some(markdown)
        };

        markdown_state_clone.set(markdown_value);
    });

    html! {
        <>
        <div>
         <form>
            <div class="mb-3">
                <label for="name" class="form-label">{"Name"}</label>
                <input type="text" class="form-control" name="name"  onchange={name_changed}/>
            </div>

            <div class="mb-3">
            <label for="Details" class="form-label">{"Details"}</label>
            <MarkdownField handle_onchange={markdown_changed}/>
        </div>
         </form>
        </div>
        </>
    }
}
