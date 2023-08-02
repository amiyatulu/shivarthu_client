use crate::components::navigation::nav::Nav;
use yew::prelude::*;
use crate::components::profile_validation::profile_validation_schelling_game::profile_validation_storage_call::get_period::GetPeriod;
use crate::components::profile_validation::profile_validation_schelling_game::unstaking_transaction_condition::ConditionalTransactionModal;
#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}

#[function_component(GetIncentives)]
pub fn get_incentives(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let spinner_state: UseStateHandle<Option<bool>> = use_state(|| None);
    let submit_done = use_state(|| false);
    let submit_done_clone = submit_done.clone();
    let spinner_clone = spinner_state.clone();

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        spinner_clone.set(Some(true));
        submit_done.set(true);
    });

    if *submit_done_clone == false {
        html! {
            <>
            <Nav/>
            <div class="container">
            <GetPeriod profile_user_account={profile_user_account.clone()}/>
            <form onsubmit={onsubmit}>
            if let Some(_value) = *spinner_state {
                <input type="submit" value="Unstaking" disabled={true} />
                <img src="img/rolling.gif" alt="loading" width="40"/>
            } else {
                <input type="submit" value="Unstaking" />
            }
            </form>
            </div>
            </>
        }
    } else {
        html! {
            <>
            <ConditionalTransactionModal profile_user_account={profile_user_account}/>

            </>
        }
    }
}
