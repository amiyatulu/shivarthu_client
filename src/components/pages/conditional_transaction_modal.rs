use gloo::console::log;
use yew::prelude::*;
use yewdux::prelude::*;
use crate::components::accounts::account_store::PhaseExists;
use crate::components::accounts::set_phrase_from_pass::SetPhraseFromPass;
use crate::components::pages::transaction_from_hook::TransactionFromHooks;

#[function_component(ConditionalTransactionModal)]
pub fn conditional_transaction() -> Html {

    let (store, _) = use_store::<PhaseExists>();

    if store.phase_exists_in_state == false {
        html! {
          <SetPhraseFromPass/>
        }
    } else {
       html! {
        <>
        <TransactionFromHooks />
        </>
       }
    }


}
