use crate::app::render::rgl::{
    shader::Shader,
    texture::{Tex, TexUnit},
};

use super::Material;
use web_sys::WebGl2RenderingContext as GL;

pub struct MatWater {
    pub shader: std::rc::Rc<Shader>,
    // Textures
    pub dudv: std::rc::Rc<Tex>,
    pub normal_map: std::rc::Rc<Tex>,
    pub refraction: std::rc::Rc<crate::app::render::rgl::Framebuffer>,
    pub reflection: std::rc::Rc<crate::app::render::rgl::Framebuffer>,
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
        camera: &crate::app::render::rgl::uniform_buffer::UniformBuffer<
            crate::app::render::CameraData,
        >,
        state: &crate::app::store::State,
    ) {
        let shader: &Shader = &self.shader;

        let dudv = TexUnit::new(gl, 0);
        let normal_map = TexUnit::new(gl, 1);
        let refraction = TexUnit::new(gl, 2);
        let refraction_depth = TexUnit::new(gl, 3);
        let reflection = TexUnit::new(gl, 4);

        self.dudv.bind_at(gl, &dudv);

        self.normal_map.bind_at(gl, &normal_map);

        self.refraction
            .bind_to_unit(gl, GL::COLOR_ATTACHMENT0, &refraction);
        self.refraction
            .bind_to_unit(gl, GL::DEPTH_ATTACHMENT, &refraction_depth);
        self.reflection
            .bind_to_unit(gl, GL::COLOR_ATTACHMENT0, &reflection);

        let refraction_texture_uni = shader.get_uniform_location(gl, "refractionTexture");
        let reflection_texture_uni = shader.get_uniform_location(gl, "reflectionTexture");
        let dudv_texture_uni = shader.get_uniform_location(gl, "dudvTexture");
        let normal_map_uni = shader.get_uniform_location(gl, "normalMap");
        let water_depth_texture_uni = shader.get_uniform_location(gl, "waterDepthTexture");
        let dudv_offset_uni = shader.get_uniform_location(gl, "dudvOffset");
        let water_reflectivity_uni = shader.get_uniform_location(gl, "waterReflectivity");
        let fresnel_strength_unit = shader.get_uniform_location(gl, "fresnelStrength");

        gl.uniform1i(refraction_texture_uni.as_ref(), refraction.uniti());
        gl.uniform1i(reflection_texture_uni.as_ref(), reflection.uniti());
        gl.uniform1i(dudv_texture_uni.as_ref(), dudv.uniti());
        gl.uniform1i(normal_map_uni.as_ref(), normal_map.uniti());
        gl.uniform1i(water_depth_texture_uni.as_ref(), refraction_depth.uniti());

        gl.uniform1f(water_reflectivity_uni.as_ref(), self.reflectivity);

        gl.uniform1f(fresnel_strength_unit.as_ref(), self.fresnel_strength);

        let seconds_elapsed = state.clock() / 1000.;
        let dudv_offset = (self.wave_speed * seconds_elapsed) % 1.;
        gl.uniform1f(dudv_offset_uni.as_ref(), dudv_offset);

        let block_index = self.shader.get_uniform_block_index(gl, "Camera");
        camera.bind_base(gl, &self.shader, block_index, 2);
    }
}
