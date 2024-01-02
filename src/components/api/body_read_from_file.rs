use gloo::console::log;
use js_sys::{Promise, Uint8Array};
use web_sys::{File, FileReader};

pub async fn process_file(file: File) {
    // Read file body
    let uint8array = read_blob_as_array_buffer(&file).await;

    log!("uint8array", uint8array);
}


pub async fn read_blob_as_array_buffer(blob: &File) -> Promise {
    let reader = FileReader::new().unwrap();
    let array_buffer_promise = Promise::new(&mut |resolve, reject| {
        reader.set_onload(Some(&resolve));
        reader.set_onerror(Some(&reject));

        reader.read_as_array_buffer(blob).unwrap();
    });

    array_buffer_promise
}
