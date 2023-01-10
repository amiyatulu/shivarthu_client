use yew::prelude::*;
use yewdux::prelude::*;
use crate::components::accounts::account_store::AccountPubStore;

#[function_component(AccountAddressView)]
pub fn account_address_view() -> Html { 
    let (store, _) = use_store::<AccountPubStore>();
  
    let address_option= store.account_address.as_deref();
    html! {
        if let Some(address) = address_option {
        <>
         {address}
        </>
      }
        
    }
}