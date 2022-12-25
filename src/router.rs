use crate::components::pages::storage::Storage;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::pages::rpc_call::Rpc;
use crate::components::pages::transaction::Transaction;
use crate::components::pages::fileupload::FileUpload;
use crate::components::accounts::add_accounts::AddAccounts;
use crate::components::accounts::account_home::AccountHome;
use crate::components::accounts::create_account::CreateAccount;

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
    #[at("/home")]
    AccountHome,
    #[at("/create-account")]
    CreateAccount,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Storage => html! { <Storage />},
        Route::Rpc => html! {<Rpc/>},
        Route::Transaction => html! {<Transaction/>},
        Route::FileUpload => html! {<FileUpload/>},
        Route::AddAccounts => html! {<AddAccounts/>},
        Route::AccountHome => html! {<AccountHome/>},
        Route::CreateAccount => html! {<CreateAccount/>},
    }
}