use crate::components::accounts::account_store::PhraseStore;
use crate::components::navigation::nav::Nav;
use crate::components::accounts::multistep_account_creation::info_warning::InfoWarning;
use crate::components::accounts::multistep_account_creation::mnemonic::Mnemonic;
use crate::components::accounts::multistep_account_creation::check_mnemonic::CheckMnemonic;
// use crate::components::pages::transaction_from_hook::TransactionFromHooks;
use stylist::{yew::styled_component, Style};
use yew::{prelude::*, virtual_dom::VNode};
use yewdux::prelude::*;
const STYLE_FILE: &str = include_str!("account_home.css");

#[styled_component(CreateAccount)]
pub fn create_account() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();
    let step_state = use_state(|| 0);
    let (_, phrase_dispatch) = use_store::<PhraseStore>();

    // let mnemonic_phrase = use_state(|| "".to_string());

    let prev_step: Callback<()> = {
        let state = step_state.clone();
        Callback::from(move |_| state.set(*state - 1))
    };

    let next_step: Callback<()> = {
        let state = step_state.clone();
        Callback::from(move |_| state.set(*state + 1))
    };
    // let mnemonic_phrase_clone = mnemonic_phrase.clone();
    let dispatch = phrase_dispatch.clone();
    let set_mnemonic_phrase = Callback::from(move |phrase:String| {
        dispatch.reduce_mut(|store| store.mnemonic_phrase = Some(phrase));
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
            <Mnemonic continue_onclick={next_step} back_onclick={prev_step}  />
            </>
        }
    };

    let step_two = {
        let next_step = next_step.clone();
        let prev_step = prev_step.clone();

        html! {
            <>
            <CheckMnemonic continue_onclick={next_step} back_onclick={prev_step} />
            </>
        }
    };

    let step_three = {
        html! {
            <>
            // <TransactionFromHooks/>
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
        2 => html! {<> {main_node(step_two)} </>}, 
        _ => html! {<> {*step_state} </>},
    }
}
