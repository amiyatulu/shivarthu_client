use crate::components::accounts::account_store::PhraseStore;
use crate::components::accounts::functions::get_from_seed;
use crate::components::accounts::hooks::commons::{TransactionReturn, TransactionReturnKind};
use crate::constants::constant::NODE_URL;
use futures::StreamExt;
use std::ops::Deref;
use subxt::error::{DispatchError, Error};
use subxt::{tx::TxStatus, OnlineClient, PolkadotConfig};
use yew::prelude::*;
use yewdux::prelude::*;

// Calling the hook example:

/*
use crate::components::accounts::hooks::custom_extrinsics_subxt_hook::use_sign_tx;

#[subxt::subxt(
    runtime_metadata_path = "./artifacts/metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]

pub mod polkadot {}

use polkadot::runtime_types::pallet_support::Content;

*/

// With in functional component

/*
let content: Content = Content::IPFS(ipfs_response_for_content_clone.as_bytes().to_vec());

let add_profile_tx = polkadot::tx().profile_validation().add_citizen(content);

let hookdata = use_sign_tx(add_profile_tx);
*/

#[hook]
pub fn use_sign_tx<T>(tx: T) -> TransactionReturn
where
    T: subxt::tx::TxPayload + 'static,
{
    let (store, _) = use_store::<PhraseStore>();
    let store_clone = store.clone();

    let transaction_hash: UseStateHandle<Option<String>> = use_state(|| None);
    let transaction_hash_clone = transaction_hash.clone();
    let transaction_error: UseStateHandle<Option<String>> = use_state(|| None);
    let dispatch_error: UseStateHandle<Option<String>> = use_state(|| None);

    let transaction_error_clone_first = transaction_error.clone();
    let transaction_error_clone_second = transaction_error.clone();
    let dispatch_error_clone = dispatch_error.clone();
    let dispatch_error_clone2 = dispatch_error.clone();

    use_effect_with(store, move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let client = OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                .await
                .unwrap();
            let phrase_option = &store_clone.mnemonic_phrase;
            if let Some(seed) = phrase_option {
                let from = get_from_seed(&seed);
                let mut result = client
                    .tx()
                    .sign_and_submit_then_watch_default(&tx, &from)
                    .await
                    .unwrap();
                while let Some(status) = result.next().await {
                    match status.unwrap() {
                        TxStatus::InFinalizedBlock(in_block) => {
                            transaction_hash_clone.set(Some(format!(
                                "Transaction {:?} is finalized in block {:?}",
                                in_block.extrinsic_hash(),
                                in_block.block_hash()
                            )));

                            gloo::console::log!(format!(
                                "Transaction {:?} is finalized in block {:?}",
                                in_block.extrinsic_hash(),
                                in_block.block_hash()
                            ));

                            let events = in_block.wait_for_success().await;
                            match events {
                                Ok(_e) => {}
                                Err(Error::Runtime(DispatchError::Module(err))) => {
                                    let details = err.details().unwrap();
                                    dispatch_error_clone.set(Some(format!(
                                        "{}:{}",
                                        details.pallet.name(),
                                        &details.variant.name
                                    )))
                                }
                                _ => {}
                            }
                        }

                        other => {
                            gloo::console::log!(format!("Status: {other:?}"));
                            // transaction_error_clone_first
                            // .set(Some(format!("Status: {other:?}")))
                        }
                    }
                }
            } else {
                transaction_error_clone_second.set(Some(format!("Seed doesnot exists")));
            }
        });
    });

    if let Some(result) = &*transaction_hash {
        TransactionReturn {
            kind: TransactionReturnKind::Finalized,
            value: format!("{}", result),
            dispatch_error: dispatch_error_clone2.deref().clone(),
        }
    } else if let Some(error) = &*transaction_error {
        TransactionReturn {
            kind: TransactionReturnKind::Error,
            value: format!("{}", error),
            dispatch_error: None,
        }
    } else {
        TransactionReturn {
            kind: TransactionReturnKind::Processing,
            value: "Processing".to_owned(),
            dispatch_error: None,
        }
    }
}
