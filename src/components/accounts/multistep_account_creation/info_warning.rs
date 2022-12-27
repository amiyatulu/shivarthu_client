use yew::prelude::*;
use stylist::{yew::styled_component, Style};



#[derive(Properties, PartialEq)]
pub struct Props {
    pub continue_onclick: Callback<()>,
}




#[styled_component(InfoWarning)]
pub fn info_warning(props: &Props) -> Html {
    let continue_onclick = props.continue_onclick.clone();
    let contin = Callback::from(move |event: MouseEvent|{
        continue_onclick.emit(());
    });

    html! {
        <button type="button" class="btn btn-warning" onclick={contin}>{"Create Account"}</button>
    }
}