use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;
use crate::components::accounts::account_store::PhraseStore;
use rand::seq::SliceRandom;
use rand::thread_rng;



#[derive(Properties, PartialEq)]
pub struct Props {
    pub continue_onclick: Callback<()>,
    pub back_onclick: Callback<()>,
}



#[function_component(CheckMnemonic)]
pub fn check_mnemonic(props: &Props) -> Html {
    let (store, _) = use_store::<PhraseStore>();
    let mnemonic_phrase = store.mnemonic_phrase.clone().unwrap_or_default();
    let mut phrase_state_vec: Vec<String> = mnemonic_phrase.split_whitespace().map(str::to_string).collect();
    let mut rng = thread_rng();
    phrase_state_vec.shuffle(&mut rng);
    let tags_state: UseStateHandle<Vec<String>> = use_state(|| vec![]);
    let phrase_state =
        use_state(|| phrase_state_vec);
    let tags_state_clone1 = tags_state.clone();
    let tags_state_clone2 = tags_state.clone();
    let phrase_state_clone = phrase_state.clone();
    let phrase_state_clone2 = phrase_state.clone();

    let remove_tag = Callback::from(move |event: MouseEvent| {
        let data = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let mut check_phrase: Vec<_> = phrase_state_clone2.to_vec();
        let value = data.trim().to_owned();
        check_phrase.push(value.clone());

        phrase_state_clone2.set(check_phrase);

        let mut tags = tags_state_clone1.to_vec();
        let index = tags.iter().position(|x| *x == value.clone()).unwrap();
        tags.remove(index);
        tags_state_clone1.set(tags);
    });

    let add_tag = Callback::from(move |event: MouseEvent| {
        let data = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let mut tags: Vec<_> = tags_state_clone2.to_vec();
        let value = data.trim().to_owned();
        tags.push(value.clone());
        tags_state_clone2.set(tags.clone());
        let mut check_phrase = phrase_state_clone.to_vec();
        let index = check_phrase
            .iter()
            .position(|x| *x == value.clone())
            .unwrap();
        check_phrase.remove(index);
        phrase_state_clone.set(check_phrase);
        log!(format!("{:?}", tags));
    });
    let continue_onclick = props.continue_onclick.clone(); 

    let contin = Callback::from(move |event: MouseEvent|{        
        continue_onclick.emit(());
    });
    let back_onclick = props.back_onclick.clone();
    let back = Callback::from(move |event: MouseEvent|{
        
        back_onclick.emit(());
    });

    html! {
        <>
        {for tags_state.iter().map(|cont| {
          html! {
            <>
            <input type="text" class="form-control bg-warning text-dark" readonly=true value={cont.clone()} onclick={remove_tag.clone()}/>
           </>
          }
        })}

        <br/>
        <br/>
        <br/>
        <br/>

        {for phrase_state.iter().map(|cont| {
            html!{
                <>
                <input type="text" class="form-control" readonly=true value={cont.clone()} onclick={add_tag.clone()}/>
                </>
            }
        })}
        </>
    }
}
