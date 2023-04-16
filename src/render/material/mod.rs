pub mod albedo;
pub mod textured_quad;
pub mod water;

use web_sys::WebGl2RenderingContext as GL;

use crate::app::store::State;

use super::{rgl::uniform_buffer::UniformBuffer, BufferedMesh, CameraData};

pub trait Material {
    fn bind_uniforms(&self, gl: &GL, camera: &UniformBuffer<CameraData>, state: &State);
}
