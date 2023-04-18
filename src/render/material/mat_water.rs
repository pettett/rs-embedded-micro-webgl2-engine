use crate::render::{
    rgl::{
        shader::Shader,
        texture::{Tex, TexUnit},
    },
    TextureUnit,
};

use super::Material;
use web_sys::WebGl2RenderingContext as GL;

pub struct MatWater {
    pub shader: std::rc::Rc<Shader>,
    // Textures
    pub dudv: std::rc::Rc<Tex>,
    pub normal_map: std::rc::Rc<Tex>,
    pub refraction: std::rc::Rc<crate::render::rgl::Framebuffer>,
    pub reflection: std::rc::Rc<crate::render::rgl::Framebuffer>,
    //Rendering params
    pub reflectivity: f32,
    pub fresnel_strength: f32,
    pub wave_speed: f32,
    pub use_refraction: bool,
    pub use_reflection: bool,
}

impl Material for MatWater {
    fn bind_uniforms(
        &self,
        gl: &web_sys::WebGl2RenderingContext,
        camera: &crate::render::rgl::uniform_buffer::UniformBuffer<crate::render::CameraData>,
        state: &crate::app::store::State,
    ) {
        let shader: &Shader = &self.shader;

        self.dudv.bind_at(
            gl,
            &TexUnit::new(gl, TextureUnit::Dudv.texture_unit() as u32),
        );

        self.normal_map.bind_at(
            gl,
            &TexUnit::new(gl, TextureUnit::NormalMap.texture_unit() as u32),
        );

        self.refraction.bind_to_unit(
            gl,
            GL::COLOR_ATTACHMENT0,
            &TexUnit::new(gl, TextureUnit::Refraction.texture_unit() as u32),
        );
        self.refraction.bind_to_unit(
            gl,
            GL::DEPTH_ATTACHMENT,
            &TexUnit::new(gl, TextureUnit::RefractionDepth.texture_unit() as u32),
        );
        self.reflection.bind_to_unit(
            gl,
            GL::COLOR_ATTACHMENT0,
            &TexUnit::new(gl, TextureUnit::Reflection.texture_unit() as u32),
        );

        let refraction_texture_uni = shader.get_uniform_location(gl, "refractionTexture");
        let reflection_texture_uni = shader.get_uniform_location(gl, "reflectionTexture");
        let dudv_texture_uni = shader.get_uniform_location(gl, "dudvTexture");
        let normal_map_uni = shader.get_uniform_location(gl, "normalMap");
        let water_depth_texture_uni = shader.get_uniform_location(gl, "waterDepthTexture");
        let dudv_offset_uni = shader.get_uniform_location(gl, "dudvOffset");
        let water_reflectivity_uni = shader.get_uniform_location(gl, "waterReflectivity");
        let fresnel_strength_unit = shader.get_uniform_location(gl, "fresnelStrength");

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

        gl.uniform1f(water_reflectivity_uni.as_ref(), self.reflectivity);

        gl.uniform1f(fresnel_strength_unit.as_ref(), self.fresnel_strength);

        let seconds_elapsed = state.clock() / 1000.;
        let dudv_offset = (self.wave_speed * seconds_elapsed) % 1.;
        gl.uniform1f(dudv_offset_uni.as_ref(), dudv_offset);

        let block_index = self.shader.get_uniform_block_index(gl, "Camera");
        camera.bind_base(gl, &self.shader, block_index, 2);
    }
}
