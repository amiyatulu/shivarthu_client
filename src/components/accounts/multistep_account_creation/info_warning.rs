use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use bip39::{Language, Mnemonic, MnemonicType};



#[derive(Properties, PartialEq)]
pub struct Props {
    pub continue_onclick: Callback<()>,
    pub set_mnemonic_phrase: Callback<String>,
}




#[styled_component(InfoWarning)]
pub fn info_warning(props: &Props) -> Html {
    let continue_onclick = props.continue_onclick.clone();
    let set_mnemonic_phrase = props.set_mnemonic_phrase.clone();
    let contin = Callback::from(move |event: MouseEvent|{
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let phrase = format!("{}",mnemonic.phrase());

        continue_onclick.emit(());
        set_mnemonic_phrase.emit(phrase);
    });

    html! {
        <button type="button" class="btn btn-warning" onclick={contin}>{"Create Account"}</button>
    }
}