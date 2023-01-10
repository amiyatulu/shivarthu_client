use crate::components::accounts::account_store::PhraseStore;
use gloo::console::log;
use crate::components::accounts::functions::get_from_seed_sr;
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
                        let pair = get_from_seed_sr(&seed);
                        let signer = PairSigner::new(pair);
                        let result = client.tx().sign_and_submit_default(&tx, &signer).await;
                        match result {
                            Ok(hash) => {
                                hash_state_clone.set(format!("{:?}", hash.clone()));
                                log!(format!("{:?}", hash));
                            }
                            Err(err) => {
                                hash_state_clone.set(format!("{:?}", err));
                                log!(format!("{:?}", err));
                            }
                        }
                    } else {
                        log!(format!("Seed doesnot exists"));
                    }
                });

                first_load.set(false);
            };
        },
        store,
    );
    hash_state.deref().clone()
}
