use crate::components::project_tips::create_project_tips_transaction::ConditionalTransactionModal;
use crate::components::navigation::nav::Nav;
use crate::services::common_services::polkadot;
use polkadot::runtime_types::project_tips::types::TippingName;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub department_id: u64,
    pub tipping_name: String,
}

#[function_component(CreateDeparmentRequiredFund)]
pub fn create_department_required_fund(props: &Props) -> Html {
    let department_id = props.department_id.clone();
    let tipping_name_string = props.tipping_name.clone();
    let tipping_name_option = TippingName::from_str(&tipping_name_string);

    let spinner_state: UseStateHandle<Option<bool>> = use_state(|| None);
    let funding_needed: UseStateHandle<Option<u128>> = use_state(|| None);

    let fund_needed_clone = funding_needed.clone();
    let fund_needed_clone2 = funding_needed.clone();
    let fund_needed_clone3 = funding_needed.clone();

    let submit_done = use_state(|| false);
    let submit_done_clone = submit_done.clone();
    let spinner_clone = spinner_state.clone();

    let error_node = match tipping_name_option {
        Some(_tipping_name) => {
            html! {<></>}
        }
        None => {
            html! { <div>{"No tip is matched"}</div>}
        }
    };

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        spinner_clone.set(Some(true));
        if fund_needed_clone2.is_some() {
            submit_done.set(true);
        }
    });

    let fund_needed_onchanged = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value()
            .parse::<u128>()
            .expect("Invalid input");
        fund_needed_clone.set(Some(value));
    });

    if *submit_done_clone == false {
        html! {
            <>
            <Nav/>
            <div class="container">
                <form onsubmit={onsubmit}>
                    <div class="mb-3">
                    {error_node}
                    <label for="fund-needed" class="form-label">{"Fund Needed:"}</label>
                    <input name={"fund-needed"} type="number" class={"form-control"} required={true} onchange={fund_needed_onchanged}/>
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
        let funding_needed = (*fund_needed_clone3).unwrap();

        html! {
            <>
            <ConditionalTransactionModal funding_needed={funding_needed} department_id={department_id} tipping_name={tipping_name_string}/>
            </>
        }
    }
}
