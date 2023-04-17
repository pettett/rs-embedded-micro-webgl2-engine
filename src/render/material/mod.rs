pub mod mat_albedo;
pub mod mat_textured_quad;
pub mod mat_water;

pub use mat_albedo::MatAlbedo;
pub use mat_water::MatWater;

use web_sys::WebGl2RenderingContext as GL;

use crate::app::store::State;

use super::{rgl::uniform_buffer::UniformBuffer, BufferedMesh, CameraData};

pub trait Material {
    fn bind_uniforms(&self, gl: &GL, camera: &UniformBuffer<CameraData>, state: &State);
}
