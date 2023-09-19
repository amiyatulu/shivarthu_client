use yew::prelude::*;
use crate::components::positive_externality_validation::change_period_positive_externality_transaction_condition::ConditionalTransactionModal;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub user_to_calculate: String,
}

#[function_component(ChangePeriodPositiveExternality)]
pub fn change_period(props: &Props) -> Html {
    let user_to_calculate = props.user_to_calculate.clone();
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
                <input type="submit" value="Change Period" />
            }
            </form>
            </>
        }
    } else {
        html! {
            <>
            <ConditionalTransactionModal user_to_calculate={user_to_calculate}/>

            </>
        }
    }
}
