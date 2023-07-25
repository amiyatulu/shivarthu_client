use crate::components::accounts::account_store::PhraseStore;
use crate::components::accounts::hooks::commons::{TransactionReturn, TransactionReturnKind};
use crate::components::accounts::functions::get_from_seed;
use futures::StreamExt;
use subxt::{tx::TxStatus, OnlineClient, PolkadotConfig};
use yew::prelude::*;
use yewdux::prelude::*;

#[hook]
pub fn use_sign_tx<T>(tx: T) -> TransactionReturn
where
    T: subxt::tx::TxPayload + 'static,
{
    let (store, _) = use_store::<PhraseStore>();
    let first_load = use_state(|| true);
    let store_clone = store.clone();

    let transaction_hash: UseStateHandle<Option<String>> = use_state(|| None);
    let transaction_hash_clone = transaction_hash.clone();
    let transaction_error: UseStateHandle<Option<String>> = use_state(|| None);

    let transaction_error_clone_first = transaction_error.clone();
    let transaction_error_clone_second = transaction_error.clone();

    use_effect_with_deps(
        move |_| {
            if *first_load {
                wasm_bindgen_futures::spawn_local(async move {
                    let client = OnlineClient::<PolkadotConfig>::from_url(
                        "ws://127.0.0.1:9944",
                    )
                    .await.unwrap();
                    let phrase_option = &store_clone.mnemonic_phrase;
                    if let Some(seed) = phrase_option {
                        let from = get_from_seed(&seed);
                        let mut result = client.tx().sign_and_submit_then_watch_default(&tx, &from).await.unwrap();
                       while let Some(status) = result.next().await {
                            match status.unwrap() {
                                TxStatus::Finalized(in_block) => {
                                    transaction_hash_clone.set(Some(format!("Transaction {:?} is finalized in block {:?}",
                                    in_block.extrinsic_hash(),
                                    in_block.block_hash())));
                                }
                                other => {
                                    transaction_error_clone_first.set(Some(format!("Status: {other:?}")))
                                }
                            }
                       }
                    } else {
                        transaction_error_clone_second.set(Some(format!("Seed doesnot exists")));
                    }
                });

                first_load.set(false);
            };
        },
        store,
    );

    if let Some(result) = &*transaction_hash {
        TransactionReturn {
            kind: TransactionReturnKind::Finalized,
            value: format!("{}", result),
        }
    } else if let Some(error) = &*transaction_error {
        TransactionReturn {
            kind: TransactionReturnKind::Error,
            value: format!("{}", error),
        }
    } else {
        TransactionReturn {
            kind: TransactionReturnKind::Processing,
            value: "Processing".to_owned(),
        }
    }
   
}
