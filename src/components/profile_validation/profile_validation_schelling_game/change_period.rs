use yew::prelude::*;
use crate::components::profile_validation::profile_validation_schelling_game::change_period_transaction_condition::ConditionalTransactionModal;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}

#[function_component(ChangePeriod)]
pub fn change_period(props: &Props) -> Html {
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
            <form onsubmit={onsubmit}>
            if let Some(_value) = *spinner_state {
                <input type="submit" value="Change Period" disabled={true} />
                <img src="img/rolling.gif" alt="loading" width="40"/>
            } else {
                <input type="submit" value="Change Period" id="change-period"/>
            }
            </form>
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
