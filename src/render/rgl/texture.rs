use crate::render::TextureUnit;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

pub struct Texture {
    pub tex: Option<WebGlTexture>,
}
#[derive(Clone, Copy)]
pub enum TexFilter {
    Nearest = GL::NEAREST as isize,
    Linear = GL::LINEAR as isize,
}

impl Texture {
    pub fn new_color(
        gl: &GL,
        width: i32,
        height: i32,
        filter: TexFilter,
    ) -> Result<Texture, JsValue> {
        let tex = gl.create_texture();
        gl.bind_texture(GL::TEXTURE_2D, tex.as_ref());

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, filter as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, filter as i32);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            width,
            height,
            0,
            GL::RGBA as u32,
            GL::UNSIGNED_BYTE,
            None,
        )?;

        Ok(Texture { tex })
    }
    pub fn new_depth(gl: &GL, width: i32, height: i32) -> Result<Texture, JsValue> {
        let depth_texture = gl.create_texture();
        gl.active_texture(TextureUnit::RefractionDepth.TEXTURE_N());
        gl.bind_texture(GL::TEXTURE_2D, depth_texture.as_ref());
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D,
            0,
            GL::DEPTH_COMPONENT16 as i32, // Internal format
            width,
            height,
            0,
            GL::DEPTH_COMPONENT as u32, // Format
            GL::UNSIGNED_SHORT,
            None,
        )?;

        Ok(Texture { tex: depth_texture })
    }
}
