use crate::components::accounts::account_store::PhraseStore;
use crate::components::accounts::hooks::commons::{TransactionReturn, TransactionReturnKind};
use crate::constants::constant::NODE_URL;
use wasm_bindgen_futures;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;
use yewdux::prelude::*;
use js_sys::Promise;

// Calling the hook example:

/* 
let promise_creator = move |node_url: &String, seed: &String| {
    let ipfs_response_clone = ipfs_response.clone();
    js_extension_binding::add_profile(
        node_url.to_owned(),
        seed.to_owned(),
        ipfs_response_clone.clone(),
    )
};

let hookdata = use_custom_extrinsic(promise_creator);
*/

#[hook]
pub fn use_custom_extrinsic(
    promise_creator: impl FnOnce(&String, &String) -> Promise + 'static,
) -> TransactionReturn {
    let (store, _) = use_store::<PhraseStore>();

    let store_clone = store.clone();
    let transaction_hash: UseStateHandle<Option<String>> = use_state(|| None);
    let transaction_hash_clone = transaction_hash.clone();
    let transaction_error: UseStateHandle<Option<String>> = use_state(|| None);

    let transaction_error_clone_first = transaction_error.clone();
    let transaction_error_clone_second = transaction_error.clone();

    use_effect_with_deps(
        move |_| {
            let phrase_option = &store_clone.mnemonic_phrase;
            if let Some(seed) = phrase_option {
                // 5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y
                let seed = seed.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let promise = promise_creator(&NODE_URL.to_owned(), &format!("{}", seed)); // Invoke the closure here to create the promise.

                    let js_result = JsFuture::from(promise).await;

                    match js_result {
                        Ok(resultvalue) => {
                            if let Some(result) = resultvalue.as_string() {
                                transaction_hash_clone.set(Some(result));
                            } else {
                                transaction_error_clone_first
                                    .set(Some("Unexpected result type".to_owned()));
                            }
                        }
                        Err(e) => {
                            transaction_error_clone_second.set(e.as_string());
                        }
                    }
                });
            }
        },
        (),
    );
    if let Some(result) = &*transaction_hash {
        TransactionReturn {
            kind: TransactionReturnKind::Finalized,
            value: format!("{}", result),
            dispatch_error: None,

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
