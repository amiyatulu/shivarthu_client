use yew::prelude::*;
use yewdux::prelude::*;
use crate::components::accounts::account_store::AccountPubStore;
use yew_router::prelude::*; 
use crate::router::Route;

#[function_component(LoginView)]
pub fn sign_out_view() -> Html { 
    let (store, _) = use_store::<AccountPubStore>();
  
    let address_option= store.account_address.as_deref();
    html! {
        if let Some(_address) = address_option {
        <>
        <li class="nav-item">
        <Link<Route> to={Route::AddProfile} classes="nav-link">{"Add Profile"}</Link<Route>> 
        </li>
        </>
      }
        
    }
}