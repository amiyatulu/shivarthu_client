use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String
}

#[function_component(CustomButton)]
pub fn custom_buttom(props: &Props) -> Html {
    html! {
        <button>{&props.label}</button>
    }
}