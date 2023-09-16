use yew::prelude::*;
use yewdux::prelude::*;




use crate::components::accounts::account_store::PhaseExists;
use crate::components::accounts::set_phrase_from_pass::SetPhraseFromPass;
use crate::components::common_component::common_transaction_return::CommonTransactionReturn;
use crate::components::accounts::hooks::custom_extrinsics_subxt_hook::use_sign_tx;
use std::str::FromStr;
use subxt::utils::AccountId32;
use crate::services::common_services::polkadot;





#[derive(Properties, PartialEq)]
pub struct Props {
    pub dest_account: String,
    pub tranfer_balance: u128,
}

#[function_component(Transaction)]
pub fn transaction(props: &Props) -> Html {
    let dest_account = props.dest_account.clone();
    let tranfer_balance = props.tranfer_balance.clone();

    let account_id32 = AccountId32::from_str(&dest_account).unwrap();

    let balance_transfer_tx = polkadot::tx().balances().transfer(subxt::utils::MultiAddress::Id(account_id32), tranfer_balance);

    let hookdata = use_sign_tx(balance_transfer_tx);

   


    html! {
        <>
        <CommonTransactionReturn hookdata={hookdata} />

        </>

    }
}


#[function_component(ConditionalTransactionModal)]
pub fn conditional_transaction(props: &Props) -> Html {
    let dest_account = props.dest_account.clone();
    let tranfer_balance = props.tranfer_balance.clone();
    let (store, _) = use_store::<PhaseExists>();

    if store.phase_exists_in_state == false {
        html! {
          <SetPhraseFromPass/>
        }
    } else {
       html! {
        <>
        <Transaction  dest_account={dest_account} tranfer_balance={tranfer_balance}/>
        </>
       }
    }
}
