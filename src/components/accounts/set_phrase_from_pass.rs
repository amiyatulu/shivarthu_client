use yew::prelude::*;
use yewdux::prelude::*;
use gloo::console::log;
use crate::components::accounts::account_store::AccountStore;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};




#[function_component(SetPhraseFromPass)]
pub fn set_phrase_from_pass() -> Html {
  let (_, auth_dispatch) = use_store::<AccountStore>();

    let password_state: UseStateHandle<Option<String>> = use_state(|| None);
    let cloned_password_state = password_state.clone();
    let cloned_password_state2 = password_state.clone();


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

    let onsubmit = auth_dispatch.reduce_mut_callback_with(move |store, event: SubmitEvent| {
      event.prevent_default();
      if cloned_password_state2.is_some() {
      if let Some(hash) = &store.hash {
        let mc = new_magic_crypt!(cloned_password_state2.as_ref().unwrap(), 256);
        let seed = mc.decrypt_base64_to_string(&hash).unwrap();
        log!(seed);
      } else {
        log!("hash does not exit");
      }
    } else {
      log!("password is not set");
    }

  });



    html! {
        <>
          <div class="modal modal-sheet position-static d-block bg-secondary py-5" tabindex="-1" role="dialog" id="passwordforphrasemodal">
            <form onsubmit={onsubmit}>
              <div class="modal-dialog" role="document">
                <div class="modal-content rounded-4 shadow">
                  <div class="modal-header border-bottom-0">
                    <h1 class="modal-title fs-5">{"Enter Password"}</h1>
                    // <button type="button" class="btn-close" aria-label="Close"></button>
                  </div>
                  <div class="modal-body py-0">
                  <label for="seed" class="form-label">{"password"}</label>
                  <input type="text" class="form-control" name="seed" onchange={password_changed}/>
                  </div>
                  <div class="modal-footer flex-column border-top-0">
                    <button type="submit"  value="Submit" class="btn btn-lg btn-primary w-100 mx-0 mb-2">{"Save changes"}</button>
                  </div>
                </div>
              </div>
            </form>
          </div>
        </>
    }
}
