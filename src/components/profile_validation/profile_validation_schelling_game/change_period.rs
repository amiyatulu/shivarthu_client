use yew::prelude::*;
use yew_router::prelude::*;
use crate::routers::validation_router::ValidationRoute;
// use crate::components::profile_validation::profile_validation_schelling_game::change_period_transaction_condition::ConditionalTransactionModal;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}

#[function_component(ChangePeriod)]
pub fn change_period(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let navigator = use_navigator().unwrap();

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let profile_user_account = profile_user_account.clone();
        navigator.push(&ValidationRoute::ProfileChangePeriod { profile_user_account:  profile_user_account})
        
    });

    html! {
        <>
        <form onsubmit={onsubmit}>
            <input type="submit" value="Change Period" id="change-period"/>
        </form>
        </>
    }
}
