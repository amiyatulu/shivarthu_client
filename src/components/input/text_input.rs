use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub class: String,
    pub name: String,
    pub required: bool,
    pub handle_onchange: Callback<String>, // New code: add handle_onchange props
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone(); // New code: clone it
    let required = props.required.clone();
    let class = props.class.clone();
    // let focused_state = use_state(|| false);
    // let focused_state_clone = focused_state.clone();

    let onchange = Callback::from(move |event: Event| {
        // New code: move it
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        handle_onchange.emit(value); // New code: emit
    });

    // let handle_focus = Callback::from(move |_event: FocusEvent| {
    //     focused_state_clone.set(true);
    // });
    html! {
        <input type="text" class={class} name={props.name.clone()} onchange={onchange}  required={required} />
    }
}
