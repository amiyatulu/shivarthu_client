use crate::components::navigation::nav::Nav;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}

use crate::components::profile_validation::profile_validation_schelling_game::profile_validation_hook::period_hook::use_get_period;


#[function_component(SchellingGame)]
pub fn schelling_game(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let period_name = use_get_period(profile_user_account);

    html! {
        <>
        </>
    }
}
