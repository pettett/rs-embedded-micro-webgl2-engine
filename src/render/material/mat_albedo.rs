use crate::render::{
    rgl::{
        shader::Shader,
        texture::{Tex, TexUnit},
    },
    TextureUnit,
};

use super::Material;

pub struct MatAlbedo {
    pub shader: std::rc::Rc<Shader>,
    pub tex: std::rc::Rc<Tex>,
}

impl Material for MatAlbedo {
    fn bind_uniforms(
        &self,
        gl: &web_sys::WebGl2RenderingContext,
        camera: &crate::render::rgl::uniform_buffer::UniformBuffer<crate::render::CameraData>,
        state: &crate::app::store::State,
    ) {
        let mesh_texture_uni = self.shader.get_uniform_location(gl, "meshTexture");

        let u = TexUnit::new(gl, TextureUnit::Stone.texture_unit() as u32);

        self.tex.bind_at(gl, &u);

        gl.uniform1i(mesh_texture_uni.as_ref(), u.unit() as i32);

        let block_index = self.shader.get_uniform_block_index(gl, "Camera");
        camera.bind_base(gl, &self.shader, block_index, 2);
    }
}
