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
use std::ops::Deref;
use std::str::FromStr;
use subxt::utils::AccountId32;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub user_to_calculate: String,
    pub amount_to_fund: u32,
}

#[derive(Properties, PartialEq)]
pub struct ExtensionProps {
    pub user_to_calculate: String,
    pub amount_to_fund: u32,
    pub account_address: String,
    pub account_source: String,
}

#[function_component(Transaction)]
pub fn transaction(props: &Props) -> Html {
    let user_to_calculate = props.user_to_calculate.clone();
    let amount_to_fund = props.amount_to_fund.clone();
    let account_id32 = AccountId32::from_str(&user_to_calculate).unwrap();

    let add_juror_stake = polkadot::tx()
        .positive_externality_validation()
        .apply_jurors_positive_externality(account_id32, amount_to_fund.into());

    let hookdata = use_sign_tx(add_juror_stake);

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

    let user_to_calculate = props.user_to_calculate.clone();
    let amount_to_fund = props.amount_to_fund.clone() as u128;

    let account_id32 = AccountId32::from_str(&user_to_calculate).unwrap();

    let add_juror_stake = polkadot::tx()
        .positive_externality_validation()
        .apply_jurors_positive_externality(account_id32, amount_to_fund.into());

    let hookdata = use_sign_tx_extension(add_juror_stake, account_address, account_source);
    html! {
        <>
           <CommonTransactionExtensionReturn hookdata={hookdata} />
        </>
    }
}

#[function_component(ConditionalTransactionExtension)]
pub fn conditional_transaction_extension(props: &Props) -> Html {
    let user_to_calculate = props.user_to_calculate.clone();
    let amount_to_fund = props.amount_to_fund.clone();
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
            <TransactionExtension user_to_calculate={user_to_calculate} amount_to_fund={amount_to_fund} account_address={account_address} account_source={account_source} />
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
    let user_to_calculate = props.user_to_calculate.clone();
    let amount_to_fund = props.amount_to_fund.clone();
    let (store, _) = use_store::<PhaseExists>();
    let (local_storage, _) = use_store::<LocalStore>();

    let sign_in_method = local_storage.sign_in_method;

    // gloo::console::log!(format!("{:?}",sign_in_method));
    if let SignInMethod::ExtensionSignIn = sign_in_method {
        html! {
            <ConditionalTransactionExtension  user_to_calculate={user_to_calculate} amount_to_fund={amount_to_fund} />
        }
    } else {
        if store.phase_exists_in_state == false {
            html! {
              <SetPhraseFromPass/>
            }
        } else {
            html! {
             <>
             <Transaction  user_to_calculate={user_to_calculate} amount_to_fund={amount_to_fund}/>
             </>
            }
        }
    }
}
