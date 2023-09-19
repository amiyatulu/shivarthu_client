use crate::components::navigation::nav::Nav;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::components::positive_externality_validation::draw_jurors_positive_externality_transaction_condition::ConditionalTransactionModal;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub user_to_calculate: String,
}

#[function_component(DrawJurorsPositiveExternality)]
pub fn draw_jurors_positive_externality(props: &Props) -> Html {
    let user_to_calculate = props.user_to_calculate.clone();
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
                <form onsubmit={onsubmit}>
                <div class="mb-3">
                <label for="Draw Jurors" class="form-label">{"Draw Jurors:"}</label>
                <input name={"iterations"} type="number" class={"form-control"} required={true} onchange={iterations_onchanged}/>
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
        let iterations = (*add_iterations_state_else_clone).unwrap();

        html! {
            <>
            <ConditionalTransactionModal iterations={iterations} user_to_calculate={user_to_calculate}/>
            </>
        }
    }
}
