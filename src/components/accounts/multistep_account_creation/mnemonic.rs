use yew::prelude::*;
use yewdux::prelude::*;
use crate::components::accounts::account_store::PhraseStore;



#[derive(Properties, PartialEq)]
pub struct Props {
    pub continue_onclick: Callback<()>,
    pub back_onclick: Callback<()>,
}


#[function_component(Mnemonic)]
pub fn mnemonic(props: &Props) -> Html  {     
    let (store, _) = use_store::<PhraseStore>();
    let continue_onclick = props.continue_onclick.clone(); 
    let mnemonic_phrase = store.mnemonic_phrase.clone().unwrap_or_default();

    let contin = Callback::from(move |event: MouseEvent|{        
        continue_onclick.emit(());
    });
    let back_onclick = props.back_onclick.clone();
    let back = Callback::from(move |event: MouseEvent|{
        
        back_onclick.emit(());
    });
    html! {
     <>
        <div>{mnemonic_phrase}</div>
        <button type="button" class="btn btn-warning" onclick={contin}>{"Continue"}</button>
        <button type="button" class="btn btn-warning" onclick={back}>{"Back"}</button>
      </>
    }
   
}