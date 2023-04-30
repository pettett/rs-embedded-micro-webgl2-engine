use super::State;
use crate::app::{
    render::{rgl::uniform_buffer::UniformBuffer, CameraData, RenderStage, WebRenderer},
    Assets, Control,
};
use web_sys::WebGl2RenderingContext as GL;

pub trait Entity {
    fn should_render(&self, stage: &RenderStage) -> bool;

    fn update(&mut self, control: &Control);

    fn render(
        &self,
        gl: &GL,
        renderer: &WebRenderer,
        camera: &UniformBuffer<CameraData>,
        clip_plane: [f32; 4],
        stage: RenderStage,
        state: &State,
        assets: &Assets,
    );
}
