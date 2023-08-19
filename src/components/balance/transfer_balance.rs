use crate::components::navigation::nav::Nav;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use crate::components::balance::transfer_balance_transaction_condition::ConditionalTransactionModal;

#[function_component(TransferBalance)]
pub fn transfer_balance() -> Html {
    let spinner_state: UseStateHandle<Option<bool>> = use_state(|| None);
    let dest_account: UseStateHandle<Option<String>> = use_state(|| None);
    let tranfer_balance: UseStateHandle<Option<u128>> = use_state(|| None);
    let submit_done = use_state(|| false);
    let submit_done_clone = submit_done.clone();

    let spinner_clone = spinner_state.clone();

    let dest_account_clone = dest_account.clone();
    let dest_account_clone2 = dest_account.clone();

    let tranfer_balance_clone = tranfer_balance.clone();
    let tranfer_balance_clone2 = tranfer_balance.clone();

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        spinner_clone.set(Some(true));
        if dest_account_clone2.is_some() && tranfer_balance_clone2.is_some() {
            submit_done.set(true);
        }
    });

    let tranfer_balance_onchanged = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value()
            .parse::<u128>()
            .expect("Invalid input");
        tranfer_balance_clone.set(Some(value));
    });

    let dest_account_onchanged = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        dest_account_clone.set(Some(value));
    });

    if *submit_done_clone == false {
        html! {
            <>
            <Nav/>
            <div class="container">
                <form onsubmit={onsubmit}>
                <div class="mb-3">
                <label for="Destination Account" class="form-label">{"Destination Account Address:"}</label>
                <input name={"destination_account"} type="text" class={"form-control"} required={true} onchange={dest_account_onchanged}/>
                </div>
                <div class="mb-3">
                <label for="Transfer Balance" class="form-label">{"Transfer Balance:"}</label>
                <input name={"tranfer_balance"} type="number" class={"form-control"} required={true} onchange={tranfer_balance_onchanged}/>
                </div>
                if let Some(_value) = *spinner_state {
                    <input type="submit" value="Submit" disabled={true} />
                    <Icon icon_id={IconId::FontAwesomeSolidSpinner} />
                } else {
                    <input type="submit" value="Submit" />
                }

                </form>
            </div>
            </>
        }
    } else {
        let dest_account = format!("{}", dest_account.as_deref().unwrap_or_default());
        let tranfer_balance = (*tranfer_balance).unwrap();

        html! {
            <>
            <ConditionalTransactionModal dest_account={dest_account} tranfer_balance={tranfer_balance}/>

            </>
        }
    }
}
