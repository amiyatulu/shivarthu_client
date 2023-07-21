macro_rules! custom_extrinsics_hook {
    ($hook_fn:ident($($arg:expr),*), $js_binding_fn:ident) => {
        use crate::components::accounts::account_store::PhraseStore;
        use crate::components::accounts::hooks::commons::{TransactionReturn, TransactionReturnKind};
        use crate::constants::constant::NODE_URL;
        use crate::js_extension_binding;
        use wasm_bindgen_futures;
        use wasm_bindgen_futures::JsFuture;
        use yew::prelude::*;
        use yewdux::prelude::*;

        #[hook]
        pub fn $hook_fn($($arg:expr),*) -> TransactionReturn {
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
                            let promise = $js_binding_fn(
                                NODE_URL.to_owned(),
                                format!("{}", seed),
                                $($arg),*
                            );

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
    };
}


// Function call
// custom_extrinsics_hook!(use_challenge_profile(profile_user_account: String, ipfs_string: String), js_extension_binding::challege_profile);
