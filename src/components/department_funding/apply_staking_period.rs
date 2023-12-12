use crate::components::department_funding::create_staking_period_transaction::ConditionalTransactionModal;
use crate::components::navigation::nav::Nav;
use crate::services::common_services::polkadot;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub department_required_fund_id: u64,
}

#[function_component(ApplyStakingPeriod)]
pub fn apply_staking_period(props: &Props) -> Html {
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
                        <input type="submit" value="Submit" disabled={true} />
                        <img src="img/rolling.gif" alt="loading" width="40"/>
                    } else {
                        <input type="submit" value="Submit" />
                    }
                </form>
            </div>
            </>
        }
    } else {
        html! {
            <>
            <ConditionalTransactionModal  department_required_fund_id={department_required_fund_id} />
            </>
        }
    }
}
