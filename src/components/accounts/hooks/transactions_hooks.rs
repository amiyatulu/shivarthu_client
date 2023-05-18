use crate::components::accounts::account_store::PhraseStore;
use crate::constants::constant::NODE_URL;
use crate::polkadot_extension_binding;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;
use yewdux::prelude::*;

pub enum TransactionReturnKind {
    Error,
    Finalized,
    InBlock,
}

pub struct TransactionReturn {
    pub kind: TransactionReturnKind,
    pub value: String,
}

#[hook]
pub fn use_balance_tranfer() -> TransactionReturn {
    let (store, _) = use_store::<PhraseStore>();

    let store_clone = store.clone();

    use_effect_with_deps(
        move |_| {
            let phrase_option = &store_clone.mnemonic_phrase;
            if let Some(seed) = phrase_option {
                // 5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y
                let seed = seed.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let promise = polkadot_extension_binding::transfer_balance(
                        NODE_URL.to_owned(),
                        format!("{}", seed),
                        "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y".to_owned(),
                        10000,
                    );

                    let js_result = JsFuture::from(promise).await;

                    match js_result {
                        Ok(resultvalue) => {
                            if let Some(result) = resultvalue.as_string() {
                                gloo::console::log!(result);
                            } else {
                                gloo::console::log!(JsValue::from_str("Unexpected result type"))
                            }
                        }
                        Err(e) => {
                            gloo::console::log!("My error msg");
                            gloo::console::log!(e);
                        }
                    }
                });
            }
        },
        (),
    );
    TransactionReturn {
        kind: TransactionReturnKind::Error,
        value: "hello world".to_owned(),
    }
}
