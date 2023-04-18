use crate::app::State;
//use crate::canvas::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::render::rgl::shader::Shader;
use crate::render::rgl::shader::ShaderKind;
use crate::render::rgl::uniform_buffer::UniformBuffer;
use crate::render::Render;
use crate::render::{BufferedMesh, CameraData};
use std::rc::Rc;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

pub struct TexturedQuad {
    /// Left most part of canvas is 0, rightmost is CANVAS_WIDTH
    left: u16,
    /// Bottom of canvas is 0, top is CANVAS_HEIGHT
    top: u16,
    /// How many pixels wide
    width: u16,
    /// How many pixels tall
    height: u16,
    /// The texture unit to use
    texture_unit: u8,
    /// The shader to use when rendering
    shader: Rc<Shader>,
}

impl TexturedQuad {
    pub fn new(
        left: u16,
        top: u16,
        width: u16,
        height: u16,
        texture_unit: u8,
        shader: Rc<Shader>,
    ) -> TexturedQuad {
        TexturedQuad {
            left,
            top,
            width,
            height,
            texture_unit,
            shader,
        }
    }
}

impl<'a> Render<'a> for TexturedQuad {
    fn shader_kind() -> ShaderKind {
        ShaderKind::TexturedQuad
    }

    fn shader(&'a self) -> Rc<Shader> {
        self.shader.clone()
    }

    fn buffer_attributes(&self, gl: &GL, state: &State) -> BufferedMesh {
        let shader = self.shader();

        let vertex_data = self.make_textured_quad_vertices(state.width, state.height);

        let vertex_data_attrib = gl.get_attrib_location(&shader.program, "vertexData");
        gl.enable_vertex_attrib_array(vertex_data_attrib as u32);

        TexturedQuad::buffer_f32_data(&gl, &vertex_data[..], vertex_data_attrib as u32, 4);

        BufferedMesh { tri_size: 0 }
    }

    fn render(
        &self,
        gl: &WebGl2RenderingContext,
        _: &BufferedMesh,
        camera: &UniformBuffer<CameraData>,
        _state: &State,
    ) {
        let shader = self.shader();

        gl.uniform1i(
            shader.get_uniform_location(gl, "u_texture").as_ref(),
            self.texture_unit as i32,
        );

        gl.draw_arrays(GL::TRIANGLES, 0, 6);
    }
}

impl TexturedQuad {
    // Combine our vertex data so that we can pass one array to the GPU
    fn make_textured_quad_vertices(&self, viewport_width: u32, viewport_height: u32) -> Vec<f32> {
        let viewport_width = viewport_width as f32;
        let viewport_height = viewport_height as f32;

        let left_x = self.left as f32 / viewport_width;
        let top_y = self.top as f32 / viewport_height;
        let right_x = (self.left as f32 + self.width as f32) / viewport_width;
        let bottom_y = (self.top as f32 - self.height as f32) / viewport_height;

        let left_x = 2.0 * left_x - 1.0;
        let right_x = 2.0 * right_x - 1.0;

        let bottom_y = 2.0 * bottom_y - 1.0;
        let top_y = 2.0 * top_y - 1.0;

        // All of the positions of our quad in screen space
        let positions = [
            left_x, top_y, // Top Left
            right_x, bottom_y, // Bottom Right
            left_x, bottom_y, // Bottom Left
            left_x, top_y, // Top Left
            right_x, top_y, // Top Right
            right_x, bottom_y, // Bottom Right
        ];

        let texture_coords = [
            0., 1., // Top left
            1., 0., // Bottom Right
            0., 0., // Bottom Left
            0., 1., // Top Left
            1., 1., // Top Right
            1., 0., // Bottom Right
        ];

        let mut vertices = vec![];

        for i in 0..positions.len() {
            // Skip odd indices
            if i % 2 == 1 {
                continue;
            }

            vertices.push(positions[i]);
            vertices.push(positions[i + 1]);
            vertices.push(texture_coords[i]);
            vertices.push(texture_coords[i + 1]);
        }

        vertices
    }
}
