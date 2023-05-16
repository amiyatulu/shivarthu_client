
use std::pin::Pin;
use std::rc::Rc;
use yew::prelude::*;


// pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T>  + 'a>>;

pub struct UseSignTxHandle {
    pub api_call: Rc<dyn Fn(String) -> Pin<Box<dyn std::future::Future<Output = Option<String>>>>>,
}


impl Clone for UseSignTxHandle {
    fn clone(&self) -> Self {
        Self {
            api_call: self.api_call.clone(),
        }
    }
}


#[hook]
pub fn use_sign_tx_handle() -> UseSignTxHandle

{
    let state: UseStateHandle<Option<String>> = use_state(|| None);
    let api_call = {
        Rc::new(move |txt:String| {
            let state = state.clone();
            Box::pin(async move { Some(txt) }) as _
        })
    };
    UseSignTxHandle { api_call }
}