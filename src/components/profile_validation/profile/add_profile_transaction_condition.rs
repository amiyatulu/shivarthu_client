use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::accounts::account_store::PhaseExists;
use crate::components::accounts::set_phrase_from_pass::SetPhraseFromPass;
use crate::components::accounts::hooks::custom_extrinsics_subxt_hook::use_sign_tx;
use crate::components::common_component::common_transaction_return::CommonTransactionReturn;

#[subxt::subxt(
    runtime_metadata_path = "./artifacts/metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]

pub mod polkadot {}

use polkadot::runtime_types::pallet_support::Content;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ipfs_response: String,
}

#[function_component(Transaction)]
pub fn transaction(props: &Props) -> Html {
    let ipfs_response = props.ipfs_response.clone();

    let ipfs_response_for_content_clone = ipfs_response.clone();

    // let promise_creator = move |node_url: &String, seed: &String| {
    //     let ipfs_response_clone = ipfs_response.clone();
    //     js_extension_binding::add_profile(
    //         node_url.to_owned(),
    //         seed.to_owned(),
    //         ipfs_response_clone.clone(),
    //     )
    // };

    // let hookdata = use_custom_extrinsic(promise_creator);

    // let hookdata = add_profile_hooks::use_add_profile(ipfs_response);

    let content: Content = Content::IPFS(ipfs_response_for_content_clone.as_bytes().to_vec());

    let add_profile_tx = polkadot::tx().profile_validation().add_citizen(content);

    let hookdata = use_sign_tx(add_profile_tx);

    html! {
        <>
        <CommonTransactionReturn hookdata={hookdata} />
        </>

    }
}

#[function_component(ConditionalTransactionModal)]
pub fn conditional_transaction(props: &Props) -> Html {
    let ipfs_response = props.ipfs_response.clone();
    let (store, _) = use_store::<PhaseExists>();

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
