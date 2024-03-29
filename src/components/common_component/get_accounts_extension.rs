use crate::services::common_services::{get_accounts, Account};
use std::ops::Deref;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub account_load: Callback<(String, String)>,
}

#[function_component(GetAccountsComponent)]
pub fn get_accounts_component(props: &Props) -> Html {
    let accounts: UseStateHandle<Result<Vec<Account>, anyhow::Error>> = use_state(|| Ok(vec![]));
    let accounts_clone = accounts.clone();
    let accounts_clone_html = accounts.clone();
    let accounts_clone_get_account = accounts.clone();
    let get_account_called = use_state(|| false);
    let get_account_called_clone = get_account_called.clone();
    let account_load = props.account_load.clone();

    let get_accounts_click = Callback::from(move |_| {
        let accounts_clone = accounts_clone.clone();
        let get_account_called_clone = get_account_called_clone.clone();
        wasm_bindgen_futures::spawn_local(async move {
            get_account_called_clone.set(true);
            let accounts_value = get_accounts().await;
            accounts_clone.set(accounts_value);
        });
    });

    let get_account = Callback::from(move |i| {
        let accounts_clone = accounts_clone_get_account.clone();
        let account_load = account_load.clone();
        match accounts_clone.deref() {
            Ok(accounts) => {
                let account: &Account = accounts.get(i).unwrap();
                let account_address = account.address.clone();
                let account_source = account.source.clone();
                account_load.emit((account_address, account_source));
            }
            Err(_err) => {}
        }
    });
    let accounts_html = match accounts_clone_html.deref() {
        Ok(accounts) => {
            if accounts.is_empty() {
                html!(<div>{"No Web3 extension accounts found. Install Talisman or the Polkadot.js extension and add an account."}</div>)
            } else {
               
                html! {
                    <>
                    <div class="mb"><b>{"Select an account you want to use for signing:"}</b></div>
                    { for accounts.iter().enumerate().map(|(i, account)| {
                        let get_account = get_account.clone();
                        html! {
                            <button onclick={move |_| {get_account.clone().emit(i);}}>
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
