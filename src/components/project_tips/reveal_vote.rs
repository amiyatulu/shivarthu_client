use crate::components::navigation::nav::Nav;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use std::ops::Deref;
use crate::components::project_tips::reveal_vote_transaction_condition::ConditionalTransactionModal;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub project_id: u64,
}

#[function_component(RevealVote)]
pub fn reveal_vote(props: &Props) -> Html {
    let project_id = props.project_id.clone();
    let spinner_state: UseStateHandle<Option<bool>> = use_state(|| None);
    let choice_state: UseStateHandle<Option<u128>> = use_state(|| None);
    let salt_string_state: UseStateHandle<Option<String>> = use_state(|| None);
    let submit_done = use_state(|| false);
    let submit_done_clone = submit_done.clone();
    let salt_string_state_clone = salt_string_state.clone();
    let salt_string_state_onchange = salt_string_state.clone();
    let salt_string_state_view = salt_string_state.clone();

    let choice_state_view = choice_state.clone();
    let choice_state_onchange = choice_state.clone();

    let choice_state_else_clone = choice_state.clone();

    let spinner_clone = spinner_state.clone();

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        spinner_clone.set(Some(true));
        if choice_state.is_some() && salt_string_state.is_some() {
            submit_done.set(true);
        }
    });

    let salt_string_onchanged = Callback::from(move |event: KeyboardEvent| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        salt_string_state_onchange.set(Some(value.clone()));
    });

    let choice_onchanged = Callback::from(move |event: KeyboardEvent| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value()
            .parse::<u128>()
            .expect("Invalid input");
        choice_state_onchange.set(Some(value.clone()));
    });

    if *submit_done_clone == false {
        html! {
            <>
            <Nav/>
            <div class="container">
                <form onsubmit={onsubmit}>
                <div class="mb-3">
                <label for="Choice" class="form-label">{"Choice:"}</label>
                <input name={"choice"} type="number" class={"form-control"} required={true} onkeyup={choice_onchanged}/>
                <label for="Salt" class="form-label">{"Salt:"}</label>
                <input name={"salt"} type="text" class={"form-control"} required={true} onkeyup={salt_string_onchanged}/>
                </div>
                if let (Some(salt), Some(choice)) = (salt_string_state_view.deref().clone(), *choice_state_view) {
                    <div>{choice}{salt}</div>
                }
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
        let salt_string = format!("{}", salt_string_state_clone.as_deref().unwrap_or_default());
        let choice = choice_state_else_clone.unwrap();

        html! {
            <>
            <ConditionalTransactionModal salt_string={salt_string} project_id={project_id} choice={choice}/>
            </>
        }
    }
}
