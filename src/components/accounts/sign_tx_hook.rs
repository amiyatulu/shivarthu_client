use crate::components::accounts::account_store::PhraseStore;
use gloo::console::log;
use sp_core::{ed25519, Pair};
use sp_keyring::AccountKeyring;
use std::ops::Deref;
use subxt::{tx::PairSigner, PolkadotConfig};
use yew::prelude::*;
use yewdux::prelude::*;

#[hook]
pub fn use_sign_tx<T>(tx: T) -> String
where
    T: subxt::tx::TxPayload + 'static,
{
    let (store, _) = use_store::<PhraseStore>();
    let first_load = use_state(|| true);
    let store_clone = store.clone();

    let hash_state = use_state(|| "".to_owned());

    fn get_from_seed(seed: &str) -> ed25519::Pair {
        ed25519::Pair::from_string(&format!("{}", seed), None)
            .expect("static values are valid; qed")
    }

    let hash_state_clone = hash_state.clone();
    use_effect_with_deps(
        move |_| {
            if *first_load {
                wasm_bindgen_futures::spawn_local(async move {
                    let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url(
                        "ws://127.0.0.1:9944",
                    )
                    .await
                    .unwrap();
                    let phrase_option = &store_clone.mnemonic_phrase;
                    if let Some(seed) = phrase_option {
                        let pair = get_from_seed(&seed);
                        let signer = PairSigner::new(AccountKeyring::Alice.pair());
                        let hash = client
                            .tx()
                            .sign_and_submit_default(&tx, &signer)
                            .await
                            .unwrap();
                        hash_state_clone.set(format!("{:?}", hash.clone()));
                        log!(format!("{:?}", hash));
                    } else {
                        log!(format!("Seed doesnot exists"));
                    }
                });

                first_load.set(false);
            }
            || {};
        },
        store,
    );
    // "hello".to_string()
    hash_state.deref().clone()
}
