use gloo::console::log;
use yew::prelude::*;

#[function_component(FileUpload)]
pub fn file_upload() -> Html {
    let ondrop = {
        Callback::from(move |event: DragEvent|{
            event.prevent_default();
            let files = event.data_transfer().unwrap().files();
            log!(files);

        })
    };
    let ondragover = {
        Callback::from(move |event: DragEvent|{
            event.prevent_default();
        })
    };
    let ondragenter = {
        Callback::from(move |event: DragEvent|{
            event.prevent_default();            
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
        </div>
    }
}