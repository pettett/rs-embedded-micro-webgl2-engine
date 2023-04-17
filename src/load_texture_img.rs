use crate::app::Assets;
use crate::render::rgl::texture::Tex;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;
use web_sys::WebGl2RenderingContext as GL;

pub fn load_texture_image(gl: Rc<GL>, assets: Rc<RefCell<Assets>>, src: String) {
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
