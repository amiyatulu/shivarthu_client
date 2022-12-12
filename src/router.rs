use crate::components::pages::storage::Storage;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::pages::rpc_call::Rpc;
use crate::components::pages::transaction::Transaction;
use crate::components::pages::fileupload::FileUpload;
use crate::components::accounts::add_accounts::AddAccounts;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/storage")]
    Storage,
    #[at("/rpc")]
    Rpc,
    #[at("/transaction")]
    Transaction,
    #[at("/upload")]
    FileUpload,
    #[at("/seed")]
    AddAccounts,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Storage => html! { <Storage />},
        Route::Rpc => html! {<Rpc/>},
        Route::Transaction => html! {<Transaction/>},
        Route::FileUpload => html! {<FileUpload/>},
        Route::AddAccounts => html! {<AddAccounts/>},
    }
}