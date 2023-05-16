use crate::components::accounts::account_store::PhraseStore;
use gloo::console::log;
use crate::components::accounts::functions::get_from_seed_sr;
use std::ops::Deref;
use subxt::{tx::PairSigner, PolkadotConfig};
use yew::prelude::*;
use yewdux::prelude::*;
use subxt::blocks::ExtrinsicEvents;
use subxt::config::WithExtrinsicParams;
use subxt::tx::BaseExtrinsicParams;
use subxt::tx::PlainTip;
use subxt::SubstrateConfig;

#[hook]
pub fn use_sign_tx_event<T>(tx: T) -> std::option::Option<&'static ExtrinsicEvents<WithExtrinsicParams<SubstrateConfig, BaseExtrinsicParams<SubstrateConfig, PlainTip>>>>
where
    T: subxt::tx::TxPayload + 'static,
{
    let (store, _) = use_store::<PhraseStore>();
    let first_load = use_state(|| true);
    let store_clone = store.clone();

    let hash_state:UseStateHandle<Option<_>> = use_state(|| None);

    let hash_state_clone = hash_state.clone();
    let hash_state_return = hash_state.clone();
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
                        let result = client
                            .tx()
                            .sign_and_submit_then_watch_default(&tx, &signer)
                            .await
                            .unwrap()
                            .wait_for_finalized()
                            .await
                            .unwrap();
                        let events = result.fetch_events().await.unwrap();
                        hash_state_clone.set(Some(events))
                    } else {
                        hash_state_clone.set(None)
                    }
                });

                first_load.set(false);
            };
        },
        store,
    );
    hash_state_return.deref()
}
