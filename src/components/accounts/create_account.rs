use std::ops::Deref;

use crate::components::navigation::nav::Nav;
use crate::components::accounts::multistep_account_creation::info_warning::InfoWarning;
use crate::components::accounts::multistep_account_creation::mnemonic::Mnemonic;
use stylist::{yew::styled_component, Style};
use yew::{prelude::*, virtual_dom::VNode};
const STYLE_FILE: &str = include_str!("account_home.css");

#[styled_component(CreateAccount)]
pub fn create_account() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();
    let step_state = use_state(|| 0);
    let mnemonic_phrase = use_state(|| "".to_string());

    let prev_step: Callback<()> = {
        let state = step_state.clone();
        Callback::from(move |_| state.set(*state - 1))
    };

    let next_step: Callback<()> = {
        let state = step_state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };
    let mnemonic_phrase_clone = mnemonic_phrase.clone();

    let set_mnemonic_phrase = Callback::from(move |phrase:String| {
            mnemonic_phrase_clone.set(phrase);
    });

    let main_node = |value: VNode| {
        html! {
           <>
            <Nav/>
                <div class={classes!("container", stylesheet)}>
                    <div class="card shadow-lg p-3 mb-5 bg-body rounded home">
                        <div class="d-grid gap-3">
                        {value}
                        </div>
                    </div>
                </div>
          </>
        }
    };

    let step_zero = {
        let next_step = next_step.clone();
        html! {
            <>
            <p> {"current count: "} {*step_state} </p>
            <InfoWarning continue_onclick={next_step} set_mnemonic_phrase={set_mnemonic_phrase} />
            </>
        }
    };

    let step_one = {
        let next_step = next_step.clone();
        let prev_step = prev_step.clone();

        html! {
            <>
            <Mnemonic continue_onclick={next_step} back_onclick={prev_step} mnemonic_phrase={mnemonic_phrase.deref().clone()}  />
            </>
        }
    };

    let step = *step_state;
    match step {
        0 => html! {
            <>
             {main_node(step_zero)}
            </>
        },
        1 => html! {<> {main_node(step_one)} </>},
        _ => html! {<> {*step_state} </>},
    }
}
