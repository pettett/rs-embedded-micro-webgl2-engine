use crate::render::{
    rgl::{shader::Shader, texture::Tex},
    TextureUnit,
};

use super::Material;

pub struct MatWater {
    pub shader: std::rc::Rc<Shader>,
    pub dudv: std::rc::Rc<Tex>,
    pub normal_map: std::rc::Rc<Tex>,
}

impl Material for MatWater {
    fn bind_uniforms(
        &self,
        gl: &web_sys::WebGl2RenderingContext,
        camera: &crate::render::rgl::uniform_buffer::UniformBuffer<crate::render::CameraData>,
        state: &crate::app::store::State,
    ) {
        let shader: &Shader = &self.shader;

        let refraction_texture_uni = shader.get_uniform_location(gl, "refractionTexture");
        let reflection_texture_uni = shader.get_uniform_location(gl, "reflectionTexture");
        let dudv_texture_uni = shader.get_uniform_location(gl, "dudvTexture");
        let normal_map_uni = shader.get_uniform_location(gl, "normalMap");
        let water_depth_texture_uni = shader.get_uniform_location(gl, "waterDepthTexture");

        gl.uniform1i(
            refraction_texture_uni.as_ref(),
            TextureUnit::Refraction.texture_unit(),
        );
        gl.uniform1i(
            reflection_texture_uni.as_ref(),
            TextureUnit::Reflection.texture_unit(),
        );
        gl.uniform1i(dudv_texture_uni.as_ref(), TextureUnit::Dudv.texture_unit());
        gl.uniform1i(
            normal_map_uni.as_ref(),
            TextureUnit::NormalMap.texture_unit(),
        );
        gl.uniform1i(
            water_depth_texture_uni.as_ref(),
            TextureUnit::RefractionDepth.texture_unit(),
        );

        let block_index = self.shader.get_uniform_block_index(gl, "Camera");
        camera.bind_base(gl, &self.shader, block_index, 2);
    }
}
