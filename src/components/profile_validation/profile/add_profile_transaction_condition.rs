use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::accounts::account_store::PhaseExists;
use crate::components::accounts::set_phrase_from_pass::SetPhraseFromPass;
use crate::components::navigation::nav::Nav;
// use crate::components::accounts::hooks::add_profile_hooks;
use crate::components::accounts::hooks::commons::TransactionReturnKind;
use crate::components::accounts::hooks::custom_extrinsics_hook::use_custom_extrinsic;
use crate::js_extension_binding;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ipfs_response: String,
}

#[function_component(Transaction)]
pub fn transaction(props: &Props) -> Html {
    let ipfs_response = props.ipfs_response.clone();


    let promise_creator = move |node_url: &String, seed: &String| {
        let ipfs_response_clone = ipfs_response.clone();
            js_extension_binding::add_profile(
                node_url.to_owned(),
                seed.to_owned(),
                ipfs_response_clone,
            )
    };

    let hookdata = use_custom_extrinsic(promise_creator);

    // let hookdata = add_profile_hooks::use_add_profile(ipfs_response);

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
