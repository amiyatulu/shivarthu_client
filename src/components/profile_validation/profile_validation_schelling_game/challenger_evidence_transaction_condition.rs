use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::accounts::account_store::PhaseExists;
use crate::components::accounts::set_phrase_from_pass::SetPhraseFromPass;
use crate::components::navigation::nav::Nav;
// use crate::components::accounts::hooks::add_profile_hooks;
use crate::components::accounts::hooks::commons::TransactionReturnKind;
// use crate::components::accounts::hooks::custom_extrinsics_hook::use_custom_extrinsic;
// use crate::js_extension_binding;
use crate::components::accounts::hooks::custom_extrinsics_subxt_hook::use_sign_tx;
use subxt::utils::AccountId32;
use std::str::FromStr;

#[subxt::subxt(
    runtime_metadata_path = "./artifacts/metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]

pub mod polkadot {}

use polkadot::runtime_types::pallet_support::Content;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub profile_user_account: String,
    pub ipfs_response: String,
}

#[function_component(Transaction)]
pub fn transaction(props: &Props) -> Html {
    let ipfs_response = props.ipfs_response.clone();
    let profile_user_account = props.profile_user_account.clone();
    let profile_user_account_id32 = AccountId32::from_str(&profile_user_account).unwrap();


    let ipfs_response_for_content_clone = ipfs_response.clone();

    let content: Content = Content::IPFS(ipfs_response_for_content_clone.as_bytes().to_vec());

    let add_profile_tx = polkadot::tx().profile_validation().challenge_profile(profile_user_account_id32, content);

    let hookdata = use_sign_tx(add_profile_tx);

    html! {
        <>
        <Nav />
            <div class="container">
                <h1>{"Transaction details"}</h1>
                <p>
                {
                    match hookdata.kind {
                        TransactionReturnKind::Finalized => {hookdata.value}
                        TransactionReturnKind::Error => {hookdata.value}
                        TransactionReturnKind::InBlock => {hookdata.value}
                        TransactionReturnKind::Processing => {hookdata.value}
                    }
                }
                </p>
            </div>
        </>

    }
}

#[function_component(ConditionalTransactionModal)]
pub fn conditional_transaction(props: &Props) -> Html {
    let ipfs_response = props.ipfs_response.clone();
    let profile_user_account = props.profile_user_account.clone();
    let (store, _) = use_store::<PhaseExists>();

    if store.phase_exists_in_state == false {
        html! {
          <SetPhraseFromPass/>
        }
    } else {
        html! {
         <>
         <Transaction  ipfs_response={ipfs_response} profile_user_account={profile_user_account}/>
         </>
        }
    }
}