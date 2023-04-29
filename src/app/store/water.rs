use crate::app::render::{
    material::{MatWater, Material},
    render_trait::Render,
    rgl::shader::ShaderKind,
    CameraData, RenderStage,
};

use super::entity::Entity;
use nalgebra::Point4;
use web_sys::WebGl2RenderingContext as GL;
#[derive(Debug, Clone)]
pub struct Water {
    pub dudv: usize,
    pub normal: usize,
    pub reflectivity: f32,
    pub fresnel_strength: f32,
    pub wave_speed: f32,
    pub use_refraction: bool,
    pub use_reflection: bool,
}

impl Entity for Water {
    fn should_render(&self, stage: &RenderStage) -> bool {
        *stage == RenderStage::Water
    }
    fn update(&mut self, control: &crate::app::Control) {}

    fn render(
        &self,
        gl: &GL,
        shader: &crate::app::render::rgl::shader::Shader,
        renderer: &crate::app::render::WebRenderer,
        camera: &crate::app::render::rgl::uniform_buffer::UniformBuffer<
            crate::app::render::CameraData,
        >,
        clip_plane: [f32; 4],
        stage: RenderStage,
        state: &super::State,
        assets: &crate::app::Assets,
    ) {
        if self.use_reflection {
            let p = state.camera().get_eye_pos();
            let flipped_y_camera = CameraData {
                view: state.camera().view_flipped_y_mat(),
                projection: state.camera().projection_mat().clone(),
                pos: Point4::new(p.x, p.y, p.z, 0.0),
            };

            renderer
                .flipped_y_camera_buffer
                .buffer(gl, &flipped_y_camera);
            renderer.render_reflection_fbo(
                gl,
                self,
                &renderer.flipped_y_camera_buffer,
                state,
                assets,
            );
        }
        if self.use_refraction {
            renderer.render_refraction_fbo(gl, self, &renderer.camera_buffer, state, assets);
        }

        if self.use_reflection || self.use_refraction {
            gl.viewport(
                0,
                0,
                state.display.width as i32,
                state.display.height as i32,
            );
            gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        }

        let water_shader = renderer.shader_sys.get_shader(&ShaderKind::Water).unwrap();
        renderer.shader_sys.use_program(gl, ShaderKind::Water);

        let water_material = MatWater {
            shader: water_shader.clone(),
            dudv: assets.get_tex(self.dudv),
            normal_map: assets.get_tex(self.normal),
            refraction: renderer.refraction_framebuffer.clone(),
            reflection: renderer.reflection_framebuffer.clone(),
            reflectivity: self.reflectivity,
            fresnel_strength: self.fresnel_strength,
            wave_speed: self.wave_speed,
            use_refraction: self.use_refraction,
            use_reflection: self.use_refraction,
        };
        let b = renderer.prepare_for_render(gl, self, water_shader, "water", state);

        water_material.bind_uniforms(gl, camera, state);

        //log::info!("Rendering Water");

        <dyn Render>::render(self, gl, &b, shader, renderer, camera, state);
    }
}
