use crate::components::accounts::account_store::PhraseStore;
use crate::components::accounts::functions::get_from_seed_sr;
use std::pin::Pin;
use std::rc::Rc;
use subxt::blocks::ExtrinsicEvents;
use subxt::config::{ SubstrateConfig, WithExtrinsicParams, polkadot::PolkadotExtrinsicParams};
use subxt::{tx::PairSigner, PolkadotConfig};
use yew::prelude::*;
use yewdux::prelude::*;
use crate::constants::constant::NODE_URL;

// pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T>  + 'a>>;

pub struct UseSignTxHandle<T>{
    pub api_call: Rc<
        dyn Fn(T) -> Pin<
            Box<
                (dyn std::future::Future<
                    Output = std::option::Option<
                        ExtrinsicEvents<
                        WithExtrinsicParams<SubstrateConfig, PolkadotExtrinsicParams<SubstrateConfig>>
                        >,
                    >,
                > + 'static),
            >,
        >,
    >,
}




#[hook]
pub fn use_sign_tx_handle<T>()-> UseSignTxHandle<T>
where
    T: Clone + subxt::tx::TxPayload
    {
    let (store, _) = use_store::<PhraseStore>();
    let store_clone = store.clone();
    let api_call = {
        let phrase_option = store_clone.mnemonic_phrase.clone();
        Rc::new(move |tx: T| {
            let phrase_option = phrase_option.clone();
            let pin = Box::pin(async move {
                let tx = tx.clone();
                let phrase_option = phrase_option.clone();
                let client =
                    subxt::client::OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                        .await
                        .unwrap();
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
                    Some(events)
                } else {
                    None
                }
            });
            pin as _
        })
    };
    UseSignTxHandle { api_call }
}
