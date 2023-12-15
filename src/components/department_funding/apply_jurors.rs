use crate::components::navigation::nav::Nav;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::components::department_funding::apply_jurors_transaction_conditions::ConditionalTransactionModal;



#[derive(Properties, PartialEq)]
pub struct Props {
    pub department_required_fund_id: u64,
}

#[function_component(AddApplyJurors)]
pub fn add_apply_jurors(props: &Props) -> Html {
    let department_required_fund_id = props.department_required_fund_id.clone();
    let spinner_state: UseStateHandle<Option<bool>> = use_state(|| None);
    let add_juror_stake_state: UseStateHandle<Option<u128>> = use_state(|| None);
    let submit_done = use_state(|| false);
    let submit_done_clone = submit_done.clone();

    let add_juror_stake_state_clone = add_juror_stake_state.clone();

    let add_juror_stake_state_clone2 = add_juror_stake_state.clone();

    let add_juror_stake_state_else_clone = add_juror_stake_state.clone();

    let spinner_clone = spinner_state.clone();

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        spinner_clone.set(Some(true));
        if add_juror_stake_state_clone2.is_some() {
            submit_done.set(true);
        }
    });

    let juror_stake_onchanged = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value()
            .parse::<u128>()
            .expect("Invalid input");
        add_juror_stake_state_clone.set(Some(value));
    });

    if *submit_done_clone == false {
        html! {
            <>
            <Nav/>
            <div class="container">
                <form onsubmit={onsubmit}>
                <div class="mb-3">
                <label for="juror-stake" class="form-label">{"Juror Stake:"}</label>
                <input name={"juror-stake"} type="number" class={"form-control"} required={true} onchange={juror_stake_onchanged}/>
                </div>
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
        let stake = (*add_juror_stake_state_else_clone).unwrap();

        html! {
            <>
            <ConditionalTransactionModal stake={stake} department_required_fund_id={department_required_fund_id}/>
            </>
        }
    }
}
