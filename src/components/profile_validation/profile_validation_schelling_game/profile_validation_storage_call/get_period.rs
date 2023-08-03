use yew::prelude::*;


use crate::components::profile_validation::profile_validation_schelling_game::profile_validation_hook::period_hook::use_get_period;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}


#[function_component(GetPeriod)]
pub fn get_period(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();

    let period_name = use_get_period(profile_user_account);
    
    

    html! {
        <>
        if period_name.is_some() {
            {"Period Name: "}{format!("{:?}", period_name.unwrap())}
        }
        <br/>

        </>
    }
}
