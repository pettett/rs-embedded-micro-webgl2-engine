use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

///Top 10 most cursed pieces of code: number 1:
pub async fn fetch(uri: &str) -> Result<Vec<u8>, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(uri, &opts)?;

    request.headers().set("Accept", "model/gltf-binary")?;

    let window = web_sys::window().unwrap();

    log::info!("Fetching...");

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    log::info!("Unpacking...");

    // Convert this other `Promise` into a rust `Future`.

    let array = Uint8Array::new(&JsFuture::from(resp.array_buffer().unwrap()).await?);

    let bytes: Vec<u8> = array.to_vec();

    // Send the JSON response back to JS.
    Ok(bytes)
}
