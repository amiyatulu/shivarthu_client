use crate::components::accounts::account_store::PhraseStore;
use std::future::Future;
use std::pin::Pin;
use yew::prelude::*;
use yewdux::prelude::*;

// futures_util::future::BoxFuture
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[hook]
pub fn use_async_hook() -> impl Fn() -> BoxFuture<'static, ()> {
    let (store, _) = use_store::<PhraseStore>();
   

    || {
        Box::pin(async {
           let store = store.clone();
        })
    }
}
