use gloo::console::log;
use web_sys::{File, FileList, HtmlInputElement};
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use stylist::{yew::styled_component, Style};
use crate::components::api::ipfs_request::ipfs_call;
use crate::components::api::select_ipfs_provider::DEFAULT_IPFS_PROVIDER;

const STYLE_FILE: &str = include_str!("fileupload.css");
#[styled_component(FileUpload)]
pub fn file_upload() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();
    let filelist: UseStateHandle<Vec<File>> = use_state(|| vec![]);
    let clone_filelist = filelist.clone();
    let ondrop = {
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
            let file = event.data_transfer().unwrap().files().unwrap().get(0);
            log!(file.clone().unwrap());
            let mut filelistvalue: Vec<_> = clone_filelist.to_vec();
            filelistvalue.push(file.clone().unwrap());
            clone_filelist.set(filelistvalue);
            wasm_bindgen_futures::spawn_local(async move {
              let response = ipfs_call(DEFAULT_IPFS_PROVIDER, file.unwrap(), "ipfs".to_owned()).await;
              log!(response);
            });
        })
    };
    let ondragover = {
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
        })
    };
    let ondragenter = {
        Callback::from(move |event: DragEvent| {
            event.prevent_default();
        })
    };

    let onchange = {
        Callback::from(move |event: Event| {
            let input: HtmlInputElement = event.target_unchecked_into();
            log!(input.files());
        })
    };
    html! {
        <div class={stylesheet}>
            <div id="wrapper">
                <p id="title">{"I am inside file upload"}</p>
                <label for="file-upload">
                    <div
                        id="drop-container"
                        ondrop={ondrop}
                        ondragover={ondragover}
                        ondragenter={ondragenter}
                    >
                    <Icon icon_id={IconId::BootstrapCloudUpload} />
                    <p>{"Drop your images here or click to select"}</p>
                    </div>
                </label>
                <input
                    id="file-upload"
                    type="file"
                    accept="image/*, video/*"
                    multiple={true}
                    onchange={onchange}
                />
                <div>{format!("{:?}", filelist)}</div>
            </div>
        </div>
    }
}
