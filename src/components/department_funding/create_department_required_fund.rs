use crate::components::navigation::nav::Nav;
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
    let tipping_name = props.tipping_name.clone();
    let spinner_state: UseStateHandle<Option<bool>> = use_state(|| None);
    let fund_needed: UseStateHandle<Option<u128>> = use_state(|| None);
    let fund_needed_clone = fund_needed.clone();

    let fund_needed_clone2 = fund_needed.clone();
    let fund_needed_clone3 = fund_needed.clone();


    let submit_done = use_state(|| false);
    let submit_done_clone = submit_done.clone();
    let spinner_clone = spinner_state.clone();

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
        let fund_needed = (*fund_needed_clone3).unwrap();

        html! {
            <>
            // <ConditionalTransactionModal fund_needed={fund_needed} department_id={department_id} tipping_name={tipping_name}/>
            </>
        }
    }
}

