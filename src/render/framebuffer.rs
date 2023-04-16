// Reflection texture can be smaller since it gets distorted by the waves.
pub static REFLECTION_TEXTURE_WIDTH: i32 = 128;
pub static REFLECTION_TEXTURE_HEIGHT: i32 = 128;

// Due to the fresnel effect when you look above the water it becomes very transparent,
// so we want a larger texture for refraction so that the objects below the water can
// be seen clearly.
pub static REFRACTION_TEXTURE_WIDTH: i32 = 512;
pub static REFRACTION_TEXTURE_HEIGHT: i32 = 512;

use std::collections::HashMap;

use crate::render::TextureUnit;
use crate::render::WebRenderer;
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

use super::rgl::renderbuffer::Renderbuffer;
use super::rgl::texture::TexFilter;
use super::rgl::texture::Texture;

pub struct Framebuffer {
    pub framebuffer: Option<WebGlFramebuffer>,
    pub textures: HashMap<u32, Texture>,
}

pub struct FramebufferBind<'a> {
    fb: &'a mut Framebuffer,
    gl: &'a WebGl2RenderingContext,
}

impl<'a> Drop for FramebufferBind<'a> {
    fn drop(&mut self) {
        self.gl.bind_framebuffer(GL::FRAMEBUFFER, None);
    }
}

impl Framebuffer {
    pub fn new(gl: &WebGl2RenderingContext) -> Framebuffer {
        let framebuffer = gl.create_framebuffer();

        Framebuffer {
            framebuffer,
            textures: HashMap::new(),
        }
    }

    pub fn bind<'a>(&'a mut self, gl: &'a WebGl2RenderingContext) -> FramebufferBind<'a> {
        gl.bind_framebuffer(GL::FRAMEBUFFER, self.framebuffer.as_ref());
        FramebufferBind { fb: self, gl }
    }
}

impl<'a> FramebufferBind<'a> {
    pub fn texture_2d(&mut self, tex: Texture, attachment: u32) {
        self.gl.framebuffer_texture_2d(
            GL::FRAMEBUFFER,
            attachment,
            GL::TEXTURE_2D,
            tex.tex.as_ref(),
            0,
        );

        self.fb.textures.insert(attachment, tex);
    }

    pub fn renderbuffer(&self, rb: Renderbuffer, attachment: u32) {
        self.gl.framebuffer_renderbuffer(
            GL::FRAMEBUFFER,
            attachment,
            GL::RENDERBUFFER,
            rb.rb.as_ref(),
        );

        //self.fb.textures.insert(attachment, tex);
    }
}

impl WebRenderer {
    pub(in crate::render) fn create_refraction_framebuffer(
        gl: &WebGl2RenderingContext,
    ) -> Result<Framebuffer, JsValue> {
        let mut framebuffer = Framebuffer::new(gl);
        {
            let mut fb = framebuffer.bind(gl);

            gl.active_texture(TextureUnit::Refraction.TEXTURE_N());

            let color_texture = Texture::new_color(
                gl,
                REFRACTION_TEXTURE_WIDTH,
                REFRACTION_TEXTURE_HEIGHT,
                TexFilter::Nearest,
            )?;

            gl.active_texture(TextureUnit::RefractionDepth.TEXTURE_N());

            let depth_texture =
                Texture::new_depth(gl, REFRACTION_TEXTURE_WIDTH, REFRACTION_TEXTURE_HEIGHT)?;

            fb.texture_2d(color_texture, GL::COLOR_ATTACHMENT0);
            fb.texture_2d(depth_texture, GL::DEPTH_ATTACHMENT);

            gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        }

        Ok(framebuffer)
    }

    pub(in crate::render) fn create_reflection_framebuffer(
        gl: &WebGl2RenderingContext,
    ) -> Result<Framebuffer, JsValue> {
        let mut framebuffer = Framebuffer::new(gl);
        {
            let mut fb = framebuffer.bind(gl);

            gl.active_texture(TextureUnit::Reflection.TEXTURE_N());

            let color_texture = Texture::new_color(
                gl,
                REFLECTION_TEXTURE_WIDTH,
                REFLECTION_TEXTURE_HEIGHT,
                TexFilter::Linear,
            )?;

            let renderbuffer =
                Renderbuffer::new(gl, REFLECTION_TEXTURE_WIDTH, REFLECTION_TEXTURE_HEIGHT);

            fb.texture_2d(color_texture, GL::COLOR_ATTACHMENT0);
            fb.renderbuffer(renderbuffer, GL::DEPTH_ATTACHMENT);

            gl.bind_renderbuffer(GL::RENDERBUFFER, None);
            gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        }
        Ok(framebuffer)
    }
}
