use crate::components::input::text_input::TextInput;
use crate::components::ipfs::upload_video::UploadVideo;
use crate::components::markdown::markdown_field::MarkdownField;
use gloo::console::log;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
struct FormData {
    pub name: String,
    pub error_msg: String,
    pub required: bool,
}

#[function_component(AddProfile)]
pub fn add_profile() -> Html {
    let name_state: UseStateHandle<Option<String>> = use_state(|| None);
    let markdown_state: UseStateHandle<Option<String>> = use_state(|| None);
    let video_cid_state: UseStateHandle<Option<String>> = use_state(|| None);
    let name_state_clone = name_state.clone();
    let markdown_state_clone = markdown_state.clone();
    let video_cid_state_clone = video_cid_state.clone();
    let video_cid_state_onsubmit = video_cid_state.clone();
    let video_error_state: UseStateHandle<Option<()>> = use_state(|| None);

    let video_error_state_clone = video_error_state.clone();
    let video_error_state_cid_changed_clone = video_error_state.clone();

    let name_onchanged = Callback::from(move |name: String| {
        let name_value: Option<String> = if name.is_empty() { None } else { Some(name) };

        name_state_clone.set(name_value);
    });

    let markdown_changed = Callback::from(move |markdown: String| {
        let markdown_value: Option<String> = if markdown.is_empty() {
            None
        } else {
            Some(markdown)
        };

        markdown_state_clone.set(markdown_value);
    });

    let video_cid_changed = Callback::from(move |video_cid: String| {
        let video_cid_value: Option<String> = if video_cid.is_empty() {
            None
        } else {
            Some(video_cid)
        };

        video_cid_state_clone.set(video_cid_value);
        video_error_state_cid_changed_clone.set(None);
    });

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        if video_cid_state_onsubmit.is_none() {
            video_error_state_clone.set(Some(()));
        } else {
            video_error_state_clone.set(None);
        }
    });

    html! {
        <>
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
                if video_error_state.is_some(){
                    <p class="alert alert-danger">{"Profile video is required"}</p>
                }

            </div>

            <div>
            if video_cid_state.is_some() {
                <div class="col-md-6 offset-md-3 text-center">
                    <video width="320" height="240" controls={true}>
                    <source src={format!("https://ipfs.io/ipfs/{}", video_cid_state.as_deref().unwrap_or_default())} type="video/mp4" />
                    {"Your browser does not support the video tag."}
                    </video>
                </div>
            }

            </div>

            <input type="submit" value="Submit" />
         </form>
        </div>
        </>
    }
}
