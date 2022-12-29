use yew::prelude::*;



#[derive(Properties, PartialEq)]
pub struct Props {
    pub continue_onclick: Callback<()>,
    pub back_onclick: Callback<()>,
    pub mnemonic_phrase: String,
}


#[function_component(Mnemonic)]
pub fn mnemonic(props: &Props) -> Html  {     
    let continue_onclick = props.continue_onclick.clone(); 
    let mnemonic_phrase = props.mnemonic_phrase.clone();

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