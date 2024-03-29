use crate::components::navigation::nav::Nav;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::components::profile_validation::profile_validation_schelling_game::profile_validation_storage_call::get_period::GetPeriod;
use crate::components::profile_validation::profile_validation_schelling_game::draw_jurors_transaction_condition::ConditionalTransactionModal;
use crate::components::profile_validation::profile_validation_schelling_game::profile_validation_rpc::drawing_period_end::DrawingPeriodEnd;
use crate::components::profile_validation::profile_validation_schelling_game::change_period::ChangePeriod;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
}

#[function_component(DrawJurors)]
pub fn draw_jurors(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let spinner_state: UseStateHandle<Option<bool>> = use_state(|| None);
    let add_iterations_state: UseStateHandle<Option<u64>> = use_state(|| None);
    let submit_done = use_state(|| false);
    let submit_done_clone = submit_done.clone();

    let add_iterations_state_clone = add_iterations_state.clone();

    let add_iterations_state_clone2 = add_iterations_state.clone();

    let add_iterations_state_else_clone = add_iterations_state.clone();

    let spinner_clone = spinner_state.clone();

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        spinner_clone.set(Some(true));
        if add_iterations_state_clone2.is_some() {
            submit_done.set(true);
        }
    });

    let iterations_onchanged = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value()
            .parse::<u64>()
            .expect("Invalid input");
        add_iterations_state_clone.set(Some(value));
    });

    if *submit_done_clone == false {
        html! {
            <>
            <Nav/>
            <div class="container">
            <GetPeriod profile_user_account={profile_user_account.clone()}/>
            <DrawingPeriodEnd profile_user_account={profile_user_account.clone()}/>
            <ChangePeriod profile_user_account={profile_user_account.clone()} />

                <form onsubmit={onsubmit}>
                <div class="mb-3">
                <label for="Draw Jurors" class="form-label">{"Draw Jurors:"}</label>
                <input name={"iterations"} type="number" class={"form-control"} required={true} onchange={iterations_onchanged} id="iterations"/>
                </div>
                if let Some(_value) = *spinner_state {
                    <input type="submit" value="Submit" disabled={true} id="draw-jurors-submit" />
                    <img src="img/rolling.gif" alt="loading" width="40"/>
                } else {
                    <input type="submit" value="Submit" id="draw-jurors-submit"/>
                }

                </form>
            </div>
            </>
        }
    } else {
        let iterations = (*add_iterations_state_else_clone).unwrap();

        html! {
            <>
            <ConditionalTransactionModal iterations={iterations} profile_user_account={profile_user_account}/>
            </>
        }
    }
}
