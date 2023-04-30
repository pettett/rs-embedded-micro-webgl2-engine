use crate::app::render::rgl::shader::Shader;
use crate::State;
use web_sys::WebGl2RenderingContext as GL;

use super::rgl::uniform_buffer::UniformBuffer;
use super::CameraData;
use super::WebRenderer;

#[derive(Copy, Clone)]
pub struct BufferedMesh {
    pub tri_size: u32,
}

pub trait Render {
    fn buffer_attributes(&self, gl: &GL, shader: &Shader, state: &State) -> BufferedMesh;

    fn render_in_water(&self) -> bool;

    fn render(
        &self,
        gl: &GL,
        buffer: &BufferedMesh,
        shader: &Shader,
        renderer: &WebRenderer,
        camera: &UniformBuffer<CameraData>,
        state: &State,
    );
}
