use yew::prelude::*;
use yewdux::prelude::*;




use crate::components::accounts::account_store::PhaseExists;
use crate::components::accounts::set_phrase_from_pass::SetPhraseFromPass;
// use crate::components::accounts::hooks::profile_validation::add_profile_stake_hooks;
use crate::components::common_component::common_transaction_return::CommonTransactionReturn;

use crate::components::accounts::hooks::custom_extrinsics_subxt_hook::use_sign_tx;
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
    pub hash: [u8; 32],
}

#[function_component(Transaction)]
pub fn transaction(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let hash = props.hash.clone();
    let account_id32 = AccountId32::from_str(&profile_user_account).unwrap();


   let commit_vote_tx = polkadot::tx().profile_validation().commit_vote(account_id32, hash);

   let hookdata = use_sign_tx(commit_vote_tx);

    html! {
        <>
        <CommonTransactionReturn hookdata={hookdata} />

        </>

    }
}


#[function_component(ConditionalTransactionModal)]
pub fn conditional_transaction(props: &Props) -> Html {
    let profile_user_account = props.profile_user_account.clone();
    let hash = props.hash.clone();
    let (store, _) = use_store::<PhaseExists>();

    if store.phase_exists_in_state == false {
        html! {
          <SetPhraseFromPass/>
        }
    } else {
       html! {
        <>
        <Transaction  profile_user_account={profile_user_account} hash={hash} />
        </>
       }
    }
}
