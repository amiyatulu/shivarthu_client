use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::accounts::account_store::PhaseExists;
use crate::components::accounts::hooks::custom_extrinsics_subxt_hook::use_sign_tx;
use crate::components::accounts::set_phrase_from_pass::SetPhraseFromPass;
use crate::components::common_component::common_transaction_return::CommonTransactionReturn;
use crate::components::common_component::custom_extrinsics_extension_hook::use_sign_tx_extension;
use crate::components::common_component::get_accounts_extension::GetAccountsComponent;
use crate::constants::local_storage::{LocalStore, SignInMethod};
use crate::services::common_services::polkadot;
use polkadot::runtime_types::pallet_support::Content;
use std::ops::Deref;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ipfs_response: String,
}

#[derive(Properties, PartialEq)]
pub struct ExtensionProps {
    pub ipfs_response: String,
    pub account_address: String,
    pub account_source: String,
}

#[function_component(Transaction)]
pub fn transaction(props: &Props) -> Html {
    let ipfs_response = props.ipfs_response.clone();

    let ipfs_response_for_content_clone = ipfs_response.clone();

    let content: Content = Content::IPFS(ipfs_response_for_content_clone.as_bytes().to_vec());

    let add_profile_tx = polkadot::tx().profile_validation().add_citizen(content);

    let hookdata = use_sign_tx(add_profile_tx);

    html! {
        <>
        <CommonTransactionReturn hookdata={hookdata} />
        </>

    }
}

#[function_component(CommonTransactionExtensionReturn)]
pub fn common_transaction_extension_return(props: &ExtensionProps) -> Html {
    let ipfs_response = props.ipfs_response.clone();
    let account_address = props.account_address.clone();
    let account_source = props.account_source.clone();

    let ipfs_response_for_content_clone = ipfs_response.clone();

    let content: Content = Content::IPFS(ipfs_response_for_content_clone.as_bytes().to_vec());

    let add_profile_tx = polkadot::tx().profile_validation().add_citizen(content);

    let hookdata = use_sign_tx_extension(add_profile_tx, account_address, account_source);
    html! {
        <>
        if hookdata.error.is_some() {
            {hookdata.error.unwrap()}
        } 
        <br/>
        if hookdata.extrinsic_success.is_some() {
            {hookdata.extrinsic_success.unwrap()}
        }
        <br/>
        if hookdata.extrinsic_error.is_some() {
            {hookdata.extrinsic_error.unwrap()}
        }

        </>
    }
}

#[function_component(ConditionalTransactionExtension)]
pub fn conditiona_transaction_extension(props: &Props) -> Html {
    let ipfs_response = props.ipfs_response.clone();
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
            <CommonTransactionExtensionReturn ipfs_response={ipfs_response} account_address={account_address} account_source={account_source} />
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
    let ipfs_response = props.ipfs_response.clone();

    let (local_storage, _) = use_store::<LocalStore>();
    let (store, _) = use_store::<PhaseExists>();

    let sign_in_method = local_storage.sign_in_method;

    // gloo::console::log!(format!("{:?}",sign_in_method));
    if let SignInMethod::ExtensionSignIn = sign_in_method {
        html! {
            <ConditionalTransactionExtension  ipfs_response={ipfs_response} />
        }
    } else {
        if store.phase_exists_in_state == false {
            html! {
              <SetPhraseFromPass/>
            }
        } else {
            html! {
             <>
             <Transaction  ipfs_response={ipfs_response}/>
             </>
            }
        }
    }
}
