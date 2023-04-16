use crate::render::{rgl::shader::Shader, TextureUnit};

use super::Material;

pub struct Albedo {
    pub shader: std::rc::Rc<Shader>,
    pub tex: TextureUnit,
}

impl Material for Albedo {
    fn bind_uniforms(
        &self,
        gl: &web_sys::WebGl2RenderingContext,
        camera: &crate::render::rgl::uniform_buffer::UniformBuffer<crate::render::CameraData>,
        state: &crate::app::store::State,
    ) {
        let mesh_texture_uni = self.shader.get_uniform_location(gl, "meshTexture");

        gl.uniform1i(mesh_texture_uni.as_ref(), self.tex.texture_unit());

        let block_index = self.shader.get_uniform_block_index(gl, "Camera");
        camera.bind_base(gl, &self.shader, block_index, 2);
    }
}
