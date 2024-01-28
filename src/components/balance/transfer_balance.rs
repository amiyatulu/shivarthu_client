use crate::components::balance::transfer_balance_transaction_condition::ConditionalTransactionModal;
use crate::components::navigation::nav::Nav;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component(TransferBalance)]
pub fn transfer_balance() -> Html {
    let spinner: UseStateHandle<Option<bool>> = use_state(|| None);
    let dest_account: UseStateHandle<Option<String>> = use_state(|| None);
    let transfer_balance: UseStateHandle<Option<u128>> = use_state(|| None);
    let submit_done = use_state(|| false);
    let submit_done_clone = submit_done.clone();

    let onsubmit = {
        let dest_account = dest_account.clone();
        let transfer_balance = transfer_balance.clone();
        let spinner = spinner.clone();
        gloo::console::log!(format!("{:?},{:?}", dest_account, transfer_balance));

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            spinner.set(Some(true));
            if dest_account.is_some() && transfer_balance.is_some() {
                gloo::console::log!(format!("inside is some {:?},{:?}", dest_account, transfer_balance));

                submit_done.set(true);
            }
        })
    };

    let transfer_balance_onchanged = {
        let transfer_balance = transfer_balance.clone();
        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<u128>()
                .expect("Invalid input");
            transfer_balance.set(Some(value));
        })
    };

    let dest_account_onchanged = {
        let dest_account = dest_account.clone();

        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            dest_account.set(Some(value));
        })
    };

    let main_view = {
        html! {
        <>
        <Nav/>
        <div class="container">
            <form onsubmit={onsubmit} id="balance-transfer-from">
            <div class="mb-3">
            <label for="Destination Account" class="form-label">{"Destination Account Address:"}</label>
            <input name="destination_account" type="text" class="form-control" required={true} oninput={dest_account_onchanged}/>
            </div>
     
            <div class="mb-3">
            <label for="Transfer Balance" class="form-label">{"Transfer Balance:"}</label>
            <input name="transfer_balance" type="text" class="form-control" required={true} oninput={transfer_balance_onchanged}/>
            </div>
            if let Some(_value) = *spinner {
                <button type="submit" disabled={true} id="tranfer-balance-submit">{"Submit"}</button>
                <Icon icon_id={IconId::FontAwesomeSolidSpinner} />
            } else {
                <button type="submit" id="tranfer-balance-submit">{"Submit"}</button>
            }

            </form>
        </div>
        </>
        }
    };
    let sign_contract_view = {
        let dest_account = dest_account.clone();
        let transfer_balance = transfer_balance.clone();
        if dest_account.is_some() && transfer_balance.is_some() {
            let dest_account = format!("{}", dest_account.as_deref().unwrap_or_default());
            let transfer_balance = (*transfer_balance).unwrap();

            html! {
                <>
                <ConditionalTransactionModal dest_account={dest_account} transfer_balance={transfer_balance}/>

                </>
            }
        } else {
            html! {
                <>
                <p>{"Dest account or transfer balance is not set"}</p>
                </>
            }
        }
    };

    let current_view = if *submit_done_clone {
        // gloo::console::log!(format!("{:?}", *submit_done_clone));

        sign_contract_view
    } else {
        // gloo::console::log!(format!("{:?}", *submit_done_clone));

        main_view
    };

    html! {
        <>
        {current_view}
        </>
    }
}
