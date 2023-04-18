use crate::app::Assets;
use crate::render::rgl::texture::Tex;
use js_sys::Uint8Array;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlImageElement;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::{Request, RequestInit, RequestMode, Response};

///Top 10 most cursed pieces of code: number 1:
pub async fn fetch(uri: &str) -> Result<Vec<u8>, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(uri, &opts)?;

    request.headers().set("Accept", "model/gltf-binary")?;

    let window = web_sys::window().unwrap();

    log::info!("Fetching {}...", uri);

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    log::info!("Unpacking response from {}...", uri);

    // Convert this other `Promise` into a rust `Future`.

    let array = Uint8Array::new(&JsFuture::from(resp.array_buffer().unwrap()).await?);

    let bytes: Vec<u8> = array.to_vec();

    // Send the JSON response back to JS.
    Ok(bytes)
}
pub fn fetch_texture_image(gl: Rc<GL>, assets: Rc<RefCell<Assets>>, src: String) {
    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
    let image_clone = Rc::clone(&image);

    let src_clone = src.clone();

    let onload = Closure::wrap(Box::new(move || {
        assets.borrow_mut().register_tex(
            src_clone.clone(),
            Tex::new_from_img(&gl, image_clone.clone()),
        );
    }) as Box<dyn Fn()>);

    let image = image.borrow_mut();

    image.set_onload(Some(onload.as_ref().unchecked_ref()));
    image.set_src(&src);

    onload.forget();
}
