use yew::prelude::*;
use yewdux::prelude::*;
use crate::components::accounts::account_store::AccountPubStore;
use crate::components::balance::balance::Balance;

#[function_component(AccountAddressView)]
pub fn account_address_view() -> Html { 
    let (store, _) = use_store::<AccountPubStore>();
  
    let address_option= store.account_address.clone();
    html! {
        if let Some(address) = address_option {
        <>
         {address.clone()}

         <Balance account_id={address.clone()}/>

        </>
      }
        
    }
}