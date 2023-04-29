use crate::app::render::buffer_f32_data;
use crate::app::render::buffer_sf32_data;
use crate::app::render::buffer_u16_indices;
use crate::app::render::rgl::shader::Shader;
use crate::app::render::rgl::shader::ShaderKind;
use crate::app::render::rgl::uniform_buffer::UniformBuffer;
use crate::app::render::BufferedMesh;
use crate::app::render::CameraData;
use crate::app::render::Render;
use crate::app::render::WebRenderer;
use crate::app::store::entity::Entity;
use crate::app::store::water::Water;
use crate::app::Assets;
use crate::app::State;
use nalgebra;
use nalgebra::{Isometry3, Matrix4, Vector3};
use std::rc::Rc;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

impl Render for Water {
    fn buffer_attributes(&self, gl: &GL, shader: &Shader, state: &State) -> BufferedMesh {
        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        gl.enable_vertex_attrib_array(pos_attrib as u32);

        // These vertices are the x and z values that create a flat square tile on the `y = 0`
        // plane. In our render function we'll scale this quad into the water size that we want.
        // x and z values, y is omitted since this is a flat surface. We set it in the vertex shader
        let vertices: [[f32; 2]; 4] = [
            [-30.5, 30.5],  // Bottom Left
            [30.5, 30.5],   // Bottom Right
            [30.5, -30.5],  // Top Right
            [-30.5, -30.5], // Top Left
        ];

        let mut indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

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
        let model_uni = shader.get_uniform_location(gl, "model");

        let pos = (0., 0.0, 0.);

        let x_scale = 18.;
        let z_scale = 18.;
        let scale = Matrix4::new_nonuniform_scaling(&Vector3::new(x_scale, 1.0, z_scale));

        let model = Isometry3::new(Vector3::new(pos.0, pos.1, pos.2), nalgebra::zero());
        let model = model.to_homogeneous();
        let model = model * scale;
        let mut model_array = [0.; 16];
        model_array.copy_from_slice(model.as_slice());
        gl.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut model_array);

        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        gl.draw_elements_with_i32(GL::TRIANGLES, 6, buffer.tri_size, 0);

        gl.disable(GL::BLEND);
    }

    fn render_in_water(&self) -> bool {
        false
    }
}
