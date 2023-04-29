use super::MeshRenderOpts;
use crate::app::render::buffer_sf32_data;
use crate::app::render::buffer_u16_indices;
use crate::app::render::buffer_u32_indices;
use crate::app::render::rgl::shader::Shader;
use crate::app::render::rgl::uniform_buffer::UniformBuffer;
use crate::app::render::BufferedMesh;
use crate::app::render::CameraData;
use crate::app::render::Render;
use crate::app::render::WebRenderer;
use crate::app::Assets;
use crate::app::State;
use gltf::mesh::util::ReadIndices;
use gltf::mesh::util::ReadTexCoords;
use gltf::{buffer::Data, Primitive};
use nalgebra;
use nalgebra::Isometry3;
use nalgebra::Scale3;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

pub struct NonSkinnedGltfMesh<'a> {
    pub mesh: &'a Primitive<'a>,
    pub buffers: &'a Vec<Data>,
    pub opts: &'a MeshRenderOpts,
}

impl<'a> Render for NonSkinnedGltfMesh<'a> {
    fn buffer_attributes(&self, gl: &GL, shader: &Shader, state: &State) -> BufferedMesh {
        let mesh: &gltf::Primitive<'a> = self.mesh;

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        let normal_attrib = gl.get_attrib_location(&shader.program, "normal");
        let tangent_attrib = gl.get_attrib_location(&shader.program, "tangent");
        let uv_attrib = gl.get_attrib_location(&shader.program, "uvs");

        let reader = mesh.reader(|buffer| Some(&self.buffers[buffer.index()]));
        if let Some(iter) = reader.read_positions() {
            let verts: Vec<[f32; 3]> = iter.collect();

            gl.enable_vertex_attrib_array(pos_attrib as u32);
            buffer_sf32_data(&gl, &verts[..], pos_attrib as u32);
        }

        if let Some(iter) = reader.read_normals() {
            let norms: Vec<[f32; 3]> = iter.collect();

            gl.enable_vertex_attrib_array(normal_attrib as u32);
            buffer_sf32_data(&gl, &norms[..], normal_attrib as u32);
        }

        if let Some(iter) = reader.read_tangents() {
            let tangents: Vec<[f32; 4]> = iter.collect();
            // W is bi-tangent sign. bitangent = cross(normal, tangent.xyz) * tangent.w

            gl.enable_vertex_attrib_array(tangent_attrib as u32);
            buffer_sf32_data(&gl, &tangents[..], tangent_attrib as u32);
        } else {
            log::warn!("Primitive {} has no tangent data", mesh.index());
        }

        if let Some(ReadTexCoords::F32(iter)) = reader.read_tex_coords(0) {
            let uvs: Vec<[f32; 2]> = iter.collect();

            gl.enable_vertex_attrib_array(uv_attrib as u32);

            buffer_sf32_data(&gl, &uvs[..], uv_attrib as u32);
        }

        //
        //
        match reader.read_indices() {
            Some(ReadIndices::U16(iter)) => {
                let indicies: Vec<u16> = iter.collect();

                buffer_u16_indices(&gl, &indicies[..]);

                BufferedMesh {
                    tri_size: GL::UNSIGNED_SHORT,
                }
            }
            Some(ReadIndices::U32(iter)) => {
                let indicies: Vec<u32> = iter.collect();

                if let Some(_) = gl.get_extension("OES_element_index_uint").unwrap() {
                    buffer_u32_indices(&gl, &indicies[..]);

                    BufferedMesh {
                        tri_size: GL::UNSIGNED_INT,
                    }
                } else {
                    panic!("No support for 32 bit indices")
                }
            }
            _ => panic!("No indices"),
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
        assets: &Assets,
    ) {
        let mesh = self.mesh;
        let opts = self.opts;

        let model_uni = shader.get_uniform_location(gl, "model");
        //let view_uni = shader.get_uniform_location(gl, "view");
        //let camera_pos_uni = shader.get_uniform_location(gl, "cameraPos");
        //let perspective_uni = shader.get_uniform_location(gl, "perspective");
        let clip_plane_uni = shader.get_uniform_location(gl, "clipPlane");

        gl.uniform4fv_with_f32_array(clip_plane_uni.as_ref(), &mut opts.clip_plane.clone()[..]);

        let model = Isometry3::new(opts.pos, opts.rot);

        let mut model_array = [0.; 16];
        model_array.copy_from_slice(
            (model.to_homogeneous() * Scale3::from(opts.scale).to_homogeneous()).as_slice(),
        );
        gl.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut model_array);

        let block_index = shader.get_uniform_block_index(gl, "Camera");
        camera.bind_base(gl, &shader, block_index, 2);

        gl.draw_elements_with_i32(
            GL::TRIANGLES,
            mesh.indices().unwrap().count() as i32,
            buffer.tri_size,
            0,
        );
    }

    fn render_in_water(&self) -> bool {
        true
    }
}
