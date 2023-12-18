use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub id: String,
    pub type_value: String
}

#[function_component(CustomButton)]
pub fn custom_buttom(props: &Props) -> Html {
    html! {
        <button id={props.id.clone()} type={props.type_value.clone()}>{&props.label}</button>
    }
}