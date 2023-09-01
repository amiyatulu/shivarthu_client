use crate::services::common_services::{
    extension_signature_for_extrinsic, get_accounts, polkadot, Account,
};
use std::ops::Deref;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(GetAccountsComponent)]
pub fn get_accounts_component() -> Html {
    let accounts: UseStateHandle<Result<Vec<Account>, anyhow::Error>> = use_state(|| Ok(vec![]));
    let accounts_clone = accounts.clone();
    let account_clone_html = accounts.clone();
    let get_account_called = use_state(|| false);
    let get_account_called_clone = get_account_called.clone();

    let get_accounts_click = Callback::from(move |_| {
        let accounts_clone = accounts_clone.clone();
        let get_account_called_clone = get_account_called_clone.clone();
        wasm_bindgen_futures::spawn_local(async move {
            get_account_called_clone.set(true);
            let accounts_value = get_accounts().await;
            accounts_clone.set(accounts_value);
        });
    });
    let accounts_html = match account_clone_html.deref() {
        Ok(accounts) => {
            if accounts.is_empty() {
                html!(<div>{"No Web3 extension accounts found. Install Talisman or the Polkadot.js extension and add an account."}</div>)
            } else {
                html! {
                    <>
                    <div class="mb"><b>{"Select an account you want to use for signing:"}</b></div>
                    { for accounts.iter().enumerate().map(|(i, account)| {
                        html! {
                            <button>
                                {&account.source} {" | "} {&account.name}<br/>
                                <small>{&account.address}</small>
                            </button>
                        }
                    }) }
                </>
                }
            }
        }
        Err(err) => html! {
            <>
            <div class="error"> {"Error: "} {err.to_string()} </div>
            </>
        },
    };
    html! {
        <>
        <div class="container">
            <button onclick={get_accounts_click}> {"=> Select an Account for Signing"} </button>
            if *get_account_called{
                {accounts_html}
            } else {
                <>
                </>
            }
        </div>
        </>
    }
}
