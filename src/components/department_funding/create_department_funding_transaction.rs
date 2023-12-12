use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::accounts::account_store::PhaseExists;
use crate::components::accounts::hooks::custom_extrinsics_subxt_hook::use_sign_tx;
use crate::components::accounts::set_phrase_from_pass::SetPhraseFromPass;
use crate::components::common_component::common_transaction_extension_return::CommonTransactionExtensionReturn;
use crate::components::common_component::common_transaction_return::CommonTransactionReturn;
use crate::components::common_component::custom_extrinsics_extension_hook::use_sign_tx_extension;
use crate::components::common_component::get_accounts_extension::GetAccountsComponent;
use crate::constants::local_storage::{LocalStore, SignInMethod};
use crate::services::common_services::polkadot;
use polkadot::runtime_types::department_funding::types::TippingName;
use std::ops::Deref;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub funding_needed: u128,
    pub department_id: u64,
    pub tipping_name: String,
}

#[derive(Properties, PartialEq)]
pub struct ExtensionProps {
    pub funding_needed: u128,
    pub department_id: u64,
    pub tipping_name: String,
    pub account_address: String,
    pub account_source: String,
}

#[function_component(Transaction)]
pub fn transaction(props: &Props) -> Html {
    let funding_needed = props.funding_needed.clone();
    let department_id = props.department_id.clone();
    let tipping_name_string = props.tipping_name.clone();

    let tipping_name = TippingName::from_str(&tipping_name_string).unwrap();

    let create_department_funding_tx = polkadot::tx()
        .department_funding()
        .create_department_required_fund(department_id, tipping_name, funding_needed);

    let hookdata = use_sign_tx(create_department_funding_tx);

    html! {
        <>
        <CommonTransactionReturn hookdata={hookdata} />
        </>
    }
}

#[function_component(TransactionExtension)]
pub fn transaction_extension(props: &ExtensionProps) -> Html {
    let account_address = props.account_address.clone();
    let account_source = props.account_source.clone();

    let funding_needed = props.funding_needed.clone();
    let department_id = props.department_id.clone();
    let tipping_name_string = props.tipping_name.clone();

    let tipping_name = TippingName::from_str(&tipping_name_string).unwrap();

    let create_department_funding_tx = polkadot::tx()
        .department_funding()
        .create_department_required_fund(department_id, tipping_name, funding_needed);

    let hookdata = use_sign_tx_extension(create_department_funding_tx, account_address, account_source);

    html! {
        <>
        <CommonTransactionExtensionReturn hookdata={hookdata} />
        </>
    }
}

#[function_component(ConditionalTransactionExtension)]
pub fn conditional_transaction_extension(props: &Props) -> Html {
    let funding_needed = props.funding_needed.clone();
    let department_id = props.department_id.clone();
    let tipping_name = props.tipping_name.clone();

    let account_address: UseStateHandle<Option<String>> = use_state(|| None);
    let account_source: UseStateHandle<Option<String>> = use_state(|| None);
    let account_address_clone = account_address.clone();
    let account_source_clone = account_source.clone();
    let account_address_clone2 = account_address.clone();
    let account_source_clone2 = account_source.clone();
    let account_load = Callback::from(move |account: (String, String)| {
        account_address_clone.set(Some(account.0));
        account_source_clone.set(Some(account.1));
    });

    if account_address.is_none() || account_source.is_none() {
        html! {
            <>
            <GetAccountsComponent account_load={account_load}/>
            </>
        }
    } else if account_address.is_some() && account_source.is_some() {
        let account_address = account_address_clone2.deref().clone().unwrap();
        let account_source = account_source_clone2.deref().clone().unwrap();

        html! {
            <>
            <TransactionExtension funding_needed={funding_needed} department_id={department_id} tipping_name={tipping_name} account_address={account_address} account_source={account_source} />
            </>
        }
    } else {
        html! {
            <>
            </>
        }
    }
}

#[function_component(ConditionalTransactionModal)]
pub fn conditional_transaction(props: &Props) -> Html {
    let funding_needed = props.funding_needed.clone();
    let department_id = props.department_id.clone();
    let tipping_name = props.tipping_name.clone();
    let (store, _) = use_store::<PhaseExists>();
    let (local_storage, _) = use_store::<LocalStore>();

    let sign_in_method = local_storage.sign_in_method;

    // gloo::console::log!(format!("{:?}",sign_in_method));
    if let SignInMethod::ExtensionSignIn = sign_in_method {
        html! {
            <ConditionalTransactionExtension  funding_needed={funding_needed} department_id={department_id} tipping_name={tipping_name}/>
        }
    } else {
        if store.phase_exists_in_state == false {
            html! {
              <SetPhraseFromPass/>
            }
        } else {
            html! {
             <>
             <Transaction  funding_needed={funding_needed} department_id={department_id} tipping_name={tipping_name}/>
             </>
            }
        }
    }
}
