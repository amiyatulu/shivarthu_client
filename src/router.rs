use crate::components::pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::pages::rpc_call::Rpc;
use crate::components::pages::transaction::Transaction;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/rpc")]
    Rpc,
    #[at("/transaction")]
    Transaction,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home />},
        Route::Rpc => html! {<Rpc/>},
        Route::Transaction => html! {<Transaction/>},
    }
}