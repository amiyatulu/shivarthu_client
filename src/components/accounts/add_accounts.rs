use crate::components::accounts::account_store::AccountStore;
use crate::components::accounts::account_store::AccountPubStore;
use crate::components::input::custom_button::CustomButton;
use crate::components::navigation::nav::Nav;
use crate::components::accounts::functions::get_from_seed_sr_result;
use gloo::console::log;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use sp_core::Pair;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;
use stylist::{yew::styled_component, Style};
const STYLE_FILE: &str = include_str!("account_home.css");



#[styled_component(AddAccounts)]
pub fn add_accounts() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    let (_, auth_dispatch) = use_store::<AccountStore>();
    let (_, dipatch_pub_account) = use_store::<AccountPubStore>();
    let seed_state: UseStateHandle<Option<String>> = use_state(|| None);
    let password_state: UseStateHandle<Option<String>> = use_state(|| None);
    let seed_error: UseStateHandle<Option<String>> = use_state(|| None);
    let successful_submission = use_state(|| false);
    let cloned_seed_state = seed_state.clone();
    let cloned_password_state = password_state.clone();
    let cloned_seed_state2 = seed_state.clone();
    let cloned_password_state2 = password_state.clone();
    let successful_submission_clone = successful_submission.clone();
    let seed_error_clone = seed_error.clone();
    


    let seed_changed = Callback::from(move |event: Event| {
        let seed = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let seed_value: Option<String> = if seed.is_empty() { None } else { Some(seed) };

        cloned_seed_state.set(seed_value);
    });

    let password_changed = Callback::from(move |event: Event| {
        let password = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let password_value: Option<String> = if password.is_empty() {
            None
        } else {
            Some(password)
        };
        cloned_password_state.set(password_value);
    });

    let onsubmit = auth_dispatch.reduce_mut_callback_with(move |store, event: SubmitEvent| {
        event.prevent_default();
        let dipatch_pub_account_clone = dipatch_pub_account.clone();
        if cloned_seed_state2.is_some() && cloned_password_state2.is_some() {
            let seed_string = cloned_seed_state2.as_ref().unwrap();
            let pair_result= get_from_seed_sr_result(seed_string);
            match pair_result {
                Ok(pair) => {
                    let account = pair.public().to_string();
                    let mc = new_magic_crypt!(cloned_password_state2.as_ref().unwrap(), 256);
                    let base64 = mc.encrypt_str_to_base64(seed_string);
                    store.hash = Some(base64);
                    dipatch_pub_account_clone.reduce_mut(|store| store.account_address = Some(account));
                    successful_submission_clone.set(true)
                }
                Err(error) => {seed_error_clone.set(Some(error))}
            }

           
        }
    });

    html! {
        <>
            <Nav/> 
            <div class={classes!("container", stylesheet)}>
                <div class="card shadow-lg p-3 mb-5 bg-body rounded home">
                    <div class="d-grid gap-3">
                        if *successful_submission == false {
                            <form onsubmit={onsubmit} autocomplete="off">
                               if seed_error.is_some() {
                                <p>{seed_error.as_deref().unwrap_or_default()}</p>
                               }
                                <div class="mb-3">
                                    <label for="seed" class="form-label">{"Seed"}</label>
                                    <input type="text" class="form-control" name="seed" onchange={seed_changed}/>
                                </div>
                                <div class="mb-3">
                                    <label for="password" class="form-label">{"Password"}</label>
                                    <input type="text" class="form-control" name="password"  onchange={password_changed}/> // New code: Sending props
                                </div>

                                <CustomButton label="Submit"/><br/>
                            </form>
                        } else {
                            <p>{"Sign in successful."}</p>
                        }
                        
                    </div>
                </div>
            </div>
        </>
    }
}
