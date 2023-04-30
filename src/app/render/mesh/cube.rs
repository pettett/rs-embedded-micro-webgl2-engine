use crate::app::render::buffer_sf32_data;
use crate::app::render::buffer_u16_indices;
use crate::app::render::rgl::shader::Shader;
use crate::app::render::rgl::uniform_buffer::UniformBuffer;
use crate::app::render::BufferedMesh;
use crate::app::render::CameraData;
use crate::app::render::Render;
use crate::app::render::WebRenderer;
use crate::app::State;
use nalgebra;
use nalgebra::{Isometry3, Matrix4, Vector3};
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

pub struct Cube {
    pub pos: Vector3<f32>,
    pub extents: Vector3<f32>,
}

impl Cube {
    pub fn new(pos: Vector3<f32>, extents: Vector3<f32>) -> Cube {
        Cube { pos, extents }
    }
}

impl Render for Cube {
    fn buffer_attributes(&self, gl: &GL, shader: &Shader, state: &State) -> BufferedMesh {
        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        gl.enable_vertex_attrib_array(pos_attrib as u32);

        // These vertices are the x and z values that create a flat square tile on the `y = 0`
        // plane. In our render function we'll scale this quad into the water size that we want.
        // x and z values, y is omitted since this is a flat surface. We set it in the vertex shader
        let vertices: [[f32; 3]; 8] = [
            [-0.5, -0.5, 0.5],  // Lower Bottom Left
            [0.5, -0.5, 0.5],   // Lower Bottom Right
            [0.5, -0.5, -0.5],  //Lower Top Right
            [-0.5, -0.5, -0.5], // Lower Top Left
            [-0.5, 0.5, 0.5],   // Upper Bottom Left
            [0.5, 0.5, 0.5],    // Upper Bottom Right
            [0.5, 0.5, -0.5],   //Upper Top Right
            [-0.5, 0.5, -0.5],  // Upper Top Left
        ];

        let mut indices: [u16; 24] = [
            0, 1, 1, 2, 2, 3, 3, 0, //lower quad
            4, 5, 5, 6, 6, 7, 7, 4, //upper quad
            0, 4, 1, 5, 2, 6, 3, 7, //bridge
        ];

        buffer_sf32_data(&gl, &vertices, pos_attrib as u32);
        buffer_u16_indices(&gl, &mut indices);

        BufferedMesh {
            tri_size: GL::UNSIGNED_SHORT,
        }
    }

    fn render(
        &self,
        gl: &WebGl2RenderingContext,
        buffer: &BufferedMesh,
        shader: &Shader,
        renderer: &WebRenderer,
        camera: &UniformBuffer<CameraData>,
        state: &State,
    ) {
        let block_index = shader.get_uniform_block_index(gl, "Camera");
        camera.bind_base(gl, &shader, block_index, 2);

        let model_uni = shader.get_uniform_location(gl, "model");

        let scale = Matrix4::new_nonuniform_scaling(&self.extents.scale(2.0));

        let model = Isometry3::new(self.pos, nalgebra::zero());
        let model = model.to_homogeneous();
        let model = model * scale;
        let mut model_array = [0.; 16];
        model_array.copy_from_slice(model.as_slice());

        gl.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut model_array);

        gl.draw_elements_with_i32(GL::LINES, 24, buffer.tri_size, 0);
    }

    fn render_in_water(&self) -> bool {
        false
    }
}
