use crate::components::input::text_input::TextInput;
use crate::components::ipfs::upload_video::UploadVideo;
use crate::components::markdown::markdown_field::MarkdownField;
use crate::components::navigation::nav::Nav;
use crate::components::profile_validation::profile::add_profile_transaction_condition::ConditionalTransactionModal;
use gloo::console::log;
use json::object;
// use subxt::tx::DynamicTxPayload;
// use wasm_bindgen::JsCast;
// use web_sys::HtmlInputElement;
use yew::prelude::*;
// use yewdux::prelude::*;
// use crate::components::accounts::hooks::sign_tx_handle::{use_sign_tx_handle} ;

use crate::components::api::ipfs_request::ipfs_call_json_string;
use crate::components::api::select_ipfs_provider::DEFAULT_IPFS_PROVIDER;
use crate::constants::constant::DEFAULT_IPFS_FETCH_PROVIDER;

#[function_component(AddProfile)]
pub fn add_profile() -> Html {
    let name_state: UseStateHandle<Option<String>> = use_state(|| None);
    let markdown_state: UseStateHandle<Option<String>> = use_state(|| None);
    let video_cid_state: UseStateHandle<Option<String>> = use_state(|| None);
    let video_error_state: UseStateHandle<Option<()>> = use_state(|| None);
    let ipfs_response: UseStateHandle<Option<String>> = use_state(|| None);
    let spinner_state: UseStateHandle<Option<bool>> = use_state(|| None);
    let video_error_state_clone = video_error_state.clone();

    let name_onchanged = {
        let name_state = name_state.clone();

        Callback::from(move |name: String| {
            let name_value: Option<String> = if name.is_empty() { None } else { Some(name) };

            name_state.set(name_value);
        })
    };

    let markdown_changed = {
        let markdown_state = markdown_state.clone();

        Callback::from(move |markdown: String| {
            let markdown_value: Option<String> = if markdown.is_empty() {
                None
            } else {
                Some(markdown)
            };

            markdown_state.set(markdown_value);
        })
    };

    let video_cid_changed = {
        let video_cid_state = video_cid_state.clone();
        let video_error_state = video_error_state.clone();

        Callback::from(move |video_cid: String| {
            let video_cid_value: Option<String> = if video_cid.is_empty() {
                None
            } else {
                Some(video_cid.clone())
            };

            video_cid_state.set(video_cid_value);
            video_error_state.set(None);
        })
    };

    let onsubmit = {
        let video_cid_state = video_cid_state.clone();
        let ipfs_response = ipfs_response.clone();
        let name_state = name_state.clone();
        let spinner_state = spinner_state.clone();
        let markdown_state = markdown_state.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            spinner_state.set(Some(true));

            if video_cid_state.is_none() {
                video_error_state.set(Some(()));
            } else {
                video_error_state.set(None);
            }

            if video_cid_state.is_some() && markdown_state.is_some() && name_state.is_some() {
                let ipfs_response = ipfs_response.clone();
                let spinner_state = spinner_state.clone();

                let name = format!("{}", name_state.as_deref().unwrap_or_default());
                let details = format!("{}", markdown_state.as_deref().unwrap_or_default());
                let profile_video_cid =
                    format!("{}", video_cid_state.as_deref().unwrap_or_default());

                let data = object! {
                      version: "1.0",
                      name: name,
                      details: details,
                      profile_video_cid: profile_video_cid,
                };
                let json_string = json::stringify(data);
                wasm_bindgen_futures::spawn_local(async move {
                    let response = ipfs_call_json_string(
                        DEFAULT_IPFS_PROVIDER,
                        &json_string,
                        "ipfs".to_owned(),
                    )
                    .await;
                    ipfs_response.set(Some(response.clone()));
                    spinner_state.set(Some(false));
                    log!(response.clone());
                });
            }
        })
    };
    if ipfs_response.is_none() {
        html! {
            <>
            <Nav />
            <div class="container">
             <form onsubmit={onsubmit}>
                <div class="mb-3">
                    <label for="name" class="form-label">{"Name"}</label>
                    <TextInput name={"name"} class={"form-control"} required={true} handle_onchange={name_onchanged}/>

                </div>

                <div class="mb-3">
                    <label for="details" class="form-label">{"Details"}</label>
                    <MarkdownField name={"details"} class={"form-control"} required={true} handle_onchange={markdown_changed}/>
                </div>

                <div class="mb-3">
                    <label for="profile-video" class="form-label">{"Video Upload"}</label>
                    <UploadVideo handle_onchange_cid={video_cid_changed} />
                    if video_error_state_clone.is_some(){
                        <p class="alert alert-danger">{"Profile video is required"}</p>
                    }

                </div>

                <div>
                if video_cid_state.is_some() {
                    <div class="col-md-6 offset-md-3 text-center" id="profile-video-load">
                        <video width="320" height="240" controls={true}>
                        <source src={format!("{}{}",DEFAULT_IPFS_FETCH_PROVIDER.address, video_cid_state.as_deref().unwrap_or_default())} type="video/mp4" />
                        {"Your browser does not support the video tag."}
                        </video>
                    </div>
                }

                </div>


                if let Some(_value) = *spinner_state {
                    <input type="submit" value="Submit" disabled={true} />
                    <img src="img/rolling.gif" alt="loading" width="40"/>
                } else {
                    <input type="submit" value="Submit" />
                }
             </form>
            </div>
            </>
        }
    } else {
        let ipfs_response = format!("{}", ipfs_response.as_deref().unwrap_or_default());
        html! {
           <ConditionalTransactionModal ipfs_response={ipfs_response} />
        }
    }
}
