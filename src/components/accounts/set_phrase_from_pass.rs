use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(SetPhraseFromPass)]
pub fn set_phrase_from_pass() -> Html {
    // let onclick = {
    //     move |_| {
    //         let timeout = Timeout::new(1_000, move || {
    //         boostrap_modal_binding::tooglePasswordForPhraseModal();
    //         });
    //     }
    // };
    html! {
        <>
        <div class="modal modal-sheet position-static d-block bg-secondary py-5" tabindex="-1" role="dialog" id="passwordforphrasemodal">
        <div class="modal-dialog" role="document">
          <div class="modal-content rounded-4 shadow">
            <div class="modal-header border-bottom-0">
              <h1 class="modal-title fs-5">{"Modal title"}</h1>
              <button type="button" class="btn-close" aria-label="Close"></button>
            </div>
            <div class="modal-body py-0">
              <p>{"This is a modal sheet, a variation of the modal that docs itself to the bottom of the viewport like the newer share sheets in iOS."}</p>
            </div>
            <div class="modal-footer flex-column border-top-0">
              <button type="button" class="btn btn-lg btn-primary w-100 mx-0 mb-2">{"Save changes"}</button>
              <button type="button" class="btn btn-lg btn-light w-100 mx-0"  data-bs-target="passwordforphrasemodal"  >{"Close"}</button>
            </div>
          </div>
        </div>
      </div>
        </>
    }
}
