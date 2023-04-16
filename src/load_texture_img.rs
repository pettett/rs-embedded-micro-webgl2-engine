use crate::render::TextureUnit;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGl2RenderingContext as GL;

pub fn load_texture_image(gl: Rc<WebGl2RenderingContext>, src: &str, texture_unit: TextureUnit) {
    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
    let image_clone = Rc::clone(&image);

    let onload = Closure::wrap(Box::new(move || {
        let texture = gl.create_texture();

        gl.active_texture(texture_unit.TEXTURE_N());

        gl.bind_texture(GL::TEXTURE_2D, texture.as_ref());

        gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);

        gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            &image_clone.borrow(),
        )
        .expect("Texture image 2d");
    }) as Box<dyn Fn()>);

    let image = image.borrow_mut();

    image.set_onload(Some(onload.as_ref().unchecked_ref()));
    image.set_src(src);

    onload.forget();
}
