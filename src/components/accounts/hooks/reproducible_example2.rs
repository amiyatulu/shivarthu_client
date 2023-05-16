use std::pin::Pin;
use std::rc::Rc;
use yew::prelude::*;


// pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T>  + 'a>>;

pub struct UseSignTxHandle {
    api_call: Rc<dyn Fn() -> Pin<Box<dyn std::future::Future<Output = Option<String>>>>>,
}

impl UseSignTxHandle {
    pub fn api_call(&self) {
        (self.api_call)();
    }
}

impl Clone for UseSignTxHandle {
    fn clone(&self) -> Self {
        Self {
            api_call: self.api_call.clone(),
        }
    }
}


#[hook]
pub fn use_sign_tx_handle(tx: String) -> UseSignTxHandle

{
    let state: UseStateHandle<Option<String>> = use_state(|| None);
    let txt = tx.clone();
    let api_call = {
        Rc::new(move || {
            let txt = txt.clone();
            let state = state.clone();
            Box::pin(async move { state.as_ref().map(|_| txt) }) as _
        })
    };
    UseSignTxHandle { api_call }
}