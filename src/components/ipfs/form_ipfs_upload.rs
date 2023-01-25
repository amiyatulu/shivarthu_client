use gloo::console::log;
use json::{self, object};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Blob;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::components::api::ipfs_request::ipfs_call_json_string;


#[function_component(FormIpfsUpload)]
pub fn form_ipfs_upload() -> Html {
    let name_state: UseStateHandle<Option<String>> = use_state(|| None);
    let name_state_clone = name_state.clone();
    let name_state_submit_clone = name_state.clone();
    let details_state: UseStateHandle<Option<String>> = use_state(|| None);
    let details_state_clone = details_state.clone();
    let details_state_submit_clone = details_state.clone();

    let on_change_name = Callback::from(move |event: Event| {
        let name = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let name_value: Option<String> = if name.is_empty() { None } else { Some(name) };

        name_state_clone.set(name_value);
    });

    let on_change_details = Callback::from(move |event: Event| {
        let details = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let details_value: Option<String> = if details.is_empty() {
            None
        } else {
            Some(details)
        };

        details_state_clone.set(details_value);
    });

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        if name_state_submit_clone.is_some() && details_state_submit_clone.is_some() {
            let name = format!("{}", name_state_submit_clone.as_deref().unwrap_or_default());
            let details = format!(
                "{}",
                details_state_submit_clone.as_deref().unwrap_or_default()
            );

            let data = object! {
                name: name,
                details: details
            };

            let json_string = json::stringify(data);
            
            // let json_jsvalue = JsValue::from_str(&json_string);
            // log!(json_jsvalue.clone());
            // let json_blob_result = Blob::new_with_str_sequence(&json_jsvalue);
            // let json_blob = json_blob_result.unwrap();

            wasm_bindgen_futures::spawn_local(async move {
                let response = ipfs_call_json_string(&json_string, "ipfs".to_owned()).await;
                log!(response.Hash);
              });




        }
    });

    html! {
        <>
         <form onsubmit={onsubmit} autocomplete="off">
                <div class="mb-3">
                    <label for="name" class="form-label">{"Name"}</label>
                    <input type="text" class="form-control" name="name" onchange={on_change_name}/>
                </div>
                <div class="mb-3">
                    <label for="detials" class="form-label">{"Details"}</label>
                    <input type="text" class="form-control" name="details"  onchange={on_change_details}/>
                </div>
                <button>{"Submit"}</button>
            </form>
        </>
    }
}
