use gloo::console::log;
use web_sys::{File, FileList, HtmlInputElement};
use yew::prelude::*;
use crate::components::api::ipfs_request::ipfs_call;

#[function_component(FileUpload)]
pub fn file_upload() -> Html {
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
              let response = ipfs_call(file.unwrap(), "ipfs".to_owned()).await;
              log!(response.Hash);
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
        <div id="wrapper">
            <p id="title">{"I am inside file upload"}</p>
            <label for="file-upload">
                <div
                    id="drop-container"
                    ondrop={ondrop}
                    ondragover={ondragover}
                    ondragenter={ondragenter}
                 >
                 <i class="fa fa-cloud-upload"></i>
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
    }
}
