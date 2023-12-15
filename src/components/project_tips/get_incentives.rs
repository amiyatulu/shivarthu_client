use crate::components::project_tips::get_incentives_transaction_condition::ConditionalTransactionModal;
use crate::components::navigation::nav::Nav;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub department_required_fund_id: u64,
}

#[function_component(GetIncentives)]
pub fn get_incentives(props: &Props) -> Html {
    let department_required_fund_id = props.department_required_fund_id.clone();
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
            <ConditionalTransactionModal department_required_fund_id={department_required_fund_id}/>

            </>
        }
    }
}
