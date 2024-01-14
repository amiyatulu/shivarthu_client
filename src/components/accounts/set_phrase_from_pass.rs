use crate::components::accounts::account_store::{AccountStore, PhraseStore, PhaseExists};
use crate::router::Route;
use gloo::console::log;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(SetPhraseFromPass)]
pub fn set_phrase_from_pass() -> Html {
    let (_, auth_dispatch_account) = use_store::<AccountStore>();
    let (_, auth_dispatch_pharase) = use_store::<PhraseStore>();
    let (_, dispatch_phase_exists) = use_store::<PhaseExists>();

    let password_state: UseStateHandle<Option<String>> = use_state(|| None);
    let hash_error_sign_in = use_state(|| false);
    let password_error: UseStateHandle<Option<String>> = use_state(|| None);
    let cloned_password_state = password_state.clone();
    let cloned_password_state2 = password_state.clone();
    let cloned_hash_error_sign_in = hash_error_sign_in.clone();
    let cloned_password_error = password_error.clone();

    let password_changed = Callback::from(move |event: Event| {
        let password = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let password_value: Option<String> = if password.is_empty() {
            None
        } else {
            log!(password.clone());
            Some(password)
        };
        cloned_password_state.set(password_value);
    });

    let onsubmit = auth_dispatch_account.reduce_mut_callback_with(move |store, event: SubmitEvent| {
        event.prevent_default();
        let dispatch_pharase_clone = auth_dispatch_pharase.clone();
        let dispatch_phase_exists_clone = dispatch_phase_exists.clone();
        if cloned_password_state2.is_some() {
            if let Some(hash) = &store.hash {
                let mc = new_magic_crypt!(cloned_password_state2.as_ref().unwrap(), 256);
                let seed = mc.decrypt_base64_to_string(&hash).unwrap();
                // log!(seed.clone());
                dispatch_pharase_clone.reduce_mut(|store| store.mnemonic_phrase = Some(seed));
                dispatch_phase_exists_clone.reduce_mut(|store| store.phase_exists_in_state = true);

            } else {
                cloned_hash_error_sign_in.set(true);
                log!("hash does not exit");
            }
        } else {
            cloned_password_error.set(Some("Password is not set".to_string()));
            log!("password is not set");
        }
    });

    html! {
        <>
          <div class="modal modal-sheet position-static d-block bg-secondary py-5" tabindex="-1" role="dialog" id="passwordforphrasemodal">
            <div class="modal-dialog" role="document">
              <div class="modal-content rounded-4 shadow">
                if *hash_error_sign_in == true {
                  <div>
                    <div class="modal-header border-bottom-0">
                        <p class="alert alert-danger w-100">{"Hash does not exists"}</p>
                    </div>
                    <div class="modal-footer flex-column border-top-0">
                        <Link<Route> to={Route::AccountHome} classes="btn btn-warning w-100">{"Sign In with your seed phrase"}</Link<Route>>
                    </div>    
                  </div>
                } else {
                    <form onsubmit={onsubmit}>
                          <div class="modal-header border-bottom-0">
                            <h1 class="modal-title fs-5">{"Enter Password"}</h1>
                            // <button type="button" class="btn-close" aria-label="Close"></button>
                          </div>
                          <div class="modal-body py-0">
                            <label for="seed" class="form-label">{"password"}</label>
                            if password_error.is_some() {
                              <p class="alert alert-danger">{password_error.as_deref().unwrap_or_default()}</p>
                            }

                            <input type="text" class="form-control" name="seed" id="input-password" onchange={password_changed}/>
                            </div>
                          <div class="modal-footer flex-column border-top-0">
                            <button type="submit"  value="Submit" class="btn btn-lg btn-primary w-100 mx-0 mb-2">{"Save changes"}</button>
                          </div>

                    </form>
                }
                  
              </div>
            </div>
          </div>
        </>
    }
}
