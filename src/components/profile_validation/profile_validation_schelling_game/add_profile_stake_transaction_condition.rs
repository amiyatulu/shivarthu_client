use yew::prelude::*;
use yewdux::prelude::*;


use crate::components::accounts::account_store::PhaseExists;
use crate::components::accounts::set_phrase_from_pass::SetPhraseFromPass;
// use crate::components::accounts::hooks::profile_validation::add_profile_stake_hooks;
use crate::components::accounts::hooks::custom_extrinsics_subxt_hook::use_sign_tx;
use crate::components::common_component::common_transaction_return::CommonTransactionReturn;
use std::str::FromStr;
use subxt::utils::AccountId32;



#[subxt::subxt(
    runtime_metadata_path = "./artifacts/metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]

pub mod polkadot {}





#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
    pub amount_to_fund: u32,
}

#[function_component(Transaction)]
pub fn transaction(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let amount_to_fund = props.amount_to_fund.clone();
    let account_id32 = AccountId32::from_str(&profile_user_account).unwrap();

    let add_profile_stake = polkadot::tx().profile_validation().add_profile_stake(account_id32, amount_to_fund.into());


    let hookdata = use_sign_tx(add_profile_stake);   

    html! {
        <>
        <CommonTransactionReturn hookdata={hookdata} />

        </>

    }
}


#[function_component(ConditionalTransactionModal)]
pub fn conditional_transaction(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let amount_to_fund = props.amount_to_fund.clone();
    let (store, _) = use_store::<PhaseExists>();

    if store.phase_exists_in_state == false {
        html! {
          <SetPhraseFromPass/>
        }
    } else {
       html! {
        <>
        <Transaction  profile_user_account={profile_user_account} amount_to_fund={amount_to_fund}/>
        </>
       }
    }
}
