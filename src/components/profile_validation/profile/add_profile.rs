use crate::components::input::text_input::TextInput;
use crate::components::ipfs::upload_video::UploadVideo;
use crate::components::markdown::markdown_field::MarkdownField;
use gloo::console::log;
use json::object;
// use wasm_bindgen::JsCast;
// use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;
use crate::components::accounts::sign_tx_hook::use_sign_tx;
use crate::components::accounts::account_store::PhaseExists;
use crate::components::accounts::set_phrase_from_pass::SetPhraseFromPass;
use crate::components::api::ipfs_request::ipfs_call_json_string;
use crate::components::api::select_ipfs_provider::DEFAULT_IPFS_PROVIDER;


#[function_component(AddProfile)]
pub fn add_profile() -> Html {
    let name_state: UseStateHandle<Option<String>> = use_state(|| None);
    let markdown_state: UseStateHandle<Option<String>> = use_state(|| None);
    let video_cid_state: UseStateHandle<Option<String>> = use_state(|| None);
    let video_error_state: UseStateHandle<Option<()>> = use_state(|| None);
    let ipfs_response: UseStateHandle<Option<String>> = use_state(|| None);

    let name_state_clone = name_state.clone();
    let name_state_onsubmit = name_state.clone();
    let markdown_state_clone = markdown_state.clone();
    let markdown_state_onsubmit = markdown_state.clone();
    let video_cid_state_clone = video_cid_state.clone();
    let video_cid_state_onsubmit = video_cid_state.clone();
    let video_error_state_clone = video_error_state.clone();
    let video_error_state_cid_changed_clone = video_error_state.clone();
    let ipfs_response_onsubmit = ipfs_response.clone();

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
        let ipfs_response_onsubmit = ipfs_response_onsubmit.clone();

        if video_cid_state_onsubmit.is_none() {
            video_error_state_clone.set(Some(()));
        } else {
            video_error_state_clone.set(None);
        }

        if video_cid_state_onsubmit.is_some()
            && markdown_state_onsubmit.is_some()
            && name_state_onsubmit.is_some()
        {
            let name = format!("{}", name_state_onsubmit.as_deref().unwrap_or_default());
            let details = format!("{}", markdown_state_onsubmit.as_deref().unwrap_or_default());
            let profile_video_cid = format!(
                "{}",
                video_cid_state_onsubmit.as_deref().unwrap_or_default()
            );

            let data = object! {
                  version: "1.0",
                  name: name,
                  details: details,
                  profile_video_cid: profile_video_cid,
            };
            let json_string = json::stringify(data);

            wasm_bindgen_futures::spawn_local(async move {
                let response =
                    ipfs_call_json_string(DEFAULT_IPFS_PROVIDER, &json_string, "ipfs".to_owned())
                        .await;
                ipfs_response_onsubmit.set(Some(response));
                // log!(response);
            });
        }
    });
    if ipfs_response.is_none() {
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
    } else {
        let ipfs_string =  format!("{}", ipfs_response.as_deref().unwrap_or_default());
        html!{
           <ConditionalTransactionModal ipfs_response={ipfs_string} />
        }
    }
    
}



#[subxt::subxt(
    runtime_metadata_path = "./artifacts/metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]
pub mod polkadot {}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ipfs_response: String,
}

#[function_component(Transaction)]
pub fn transaction(props: &Props) -> Html {
    let ipfs_response = props.ipfs_response.clone();
    let profile_data = ipfs_response.as_bytes().to_vec();
    let tx = polkadot::tx().template_module().add_citizen(profile_data);

    let hash = use_sign_tx(tx);

    html! {
        <>
            <h1>{"Transaction details"}</h1>
            <p>{hash}</p>
        </>

    }
}


#[function_component(ConditionalTransactionModal)]
pub fn conditional_transaction(props: &Props) -> Html {
    let ipfs_response = props.ipfs_response.clone();
    let (store, _) = use_store::<PhaseExists>();

    if store.phase_exists_in_state == false {
        html! {
          <SetPhraseFromPass/>
        }
    } else {
       html! {
        <>
        <Transaction  ipfs_response={ipfs_response}/>
        </>
       }
    }
}
