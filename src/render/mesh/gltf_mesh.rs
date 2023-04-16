use crate::app::State;
use crate::render::rgl::shader::Shader;
use crate::render::rgl::shader::ShaderKind;
use crate::render::rgl::uniform_buffer::UniformBuffer;
use crate::render::BufferedMesh;
use crate::render::CameraData;
use crate::render::Render;
use crate::render::TextureUnit;
use gltf::mesh::util::ReadIndices;
use gltf::mesh::util::ReadTexCoords;
use gltf::{buffer::Data, Primitive};
use nalgebra;
use nalgebra::{Isometry3, Vector3};
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

use super::MeshRenderOpts;

pub struct NonSkinnedGltfMesh<'a> {
    pub mesh: &'a Primitive<'a>,
    pub buffers: &'a Vec<Data>,
    pub shader: &'a Shader,
    pub opts: &'a MeshRenderOpts,
}

impl<'a> Render<'a> for NonSkinnedGltfMesh<'a> {
    fn shader_kind() -> ShaderKind {
        ShaderKind::NonSkinnedMesh
    }

    fn shader(&'a self) -> &'a Shader {
        &self.shader
    }

    fn buffer_attributes(&self, gl: &WebGl2RenderingContext) -> BufferedMesh {
        let shader = self.shader();
        let mesh: &gltf::Primitive<'a> = self.mesh;

        let pos_attrib = gl.get_attrib_location(&shader.program, "position");
        let normal_attrib = gl.get_attrib_location(&shader.program, "normal");
        let uv_attrib = gl.get_attrib_location(&shader.program, "uvs");

        let reader = mesh.reader(|buffer| Some(&self.buffers[buffer.index()]));
        if let Some(iter) = reader.read_positions() {
            let verts: Vec<[f32; 3]> = iter.collect();

            gl.enable_vertex_attrib_array(pos_attrib as u32);
            NonSkinnedGltfMesh::buffer_sf32_data(&gl, &verts[..], pos_attrib as u32);
        }

        if let Some(iter) = reader.read_normals() {
            let norms: Vec<[f32; 3]> = iter.collect();

            gl.enable_vertex_attrib_array(normal_attrib as u32);
            NonSkinnedGltfMesh::buffer_sf32_data(&gl, &norms[..], normal_attrib as u32);
        }

        if let Some(ReadTexCoords::F32(iter)) = reader.read_tex_coords(0) {
            let uvs: Vec<[f32; 2]> = iter.collect();

            gl.enable_vertex_attrib_array(uv_attrib as u32);

            NonSkinnedGltfMesh::buffer_sf32_data(&gl, &uvs[..], uv_attrib as u32);
        }

        //
        //
        match reader.read_indices() {
            Some(ReadIndices::U16(iter)) => {
                let indicies: Vec<u16> = iter.collect();

                NonSkinnedGltfMesh::buffer_u16_indices(&gl, &indicies[..]);

                BufferedMesh {
                    tri_size: GL::UNSIGNED_SHORT,
                }
            }
            Some(ReadIndices::U32(iter)) => {
                let indicies: Vec<u32> = iter.collect();

                if let Some(_) = gl.get_extension("OES_element_index_uint").unwrap() {
                    NonSkinnedGltfMesh::buffer_u32_indices(&gl, &indicies[..]);

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
        camera: &UniformBuffer<CameraData>,
        state: &State,
    ) {
        let shader = self.shader();

        let mesh = self.mesh;
        let opts = self.opts;

        let model_uni = shader.get_uniform_location(gl, "model");
        //let view_uni = shader.get_uniform_location(gl, "view");
        //let camera_pos_uni = shader.get_uniform_location(gl, "cameraPos");
        //let perspective_uni = shader.get_uniform_location(gl, "perspective");
        let clip_plane_uni = shader.get_uniform_location(gl, "clipPlane");
        let mesh_texture_uni = shader.get_uniform_location(gl, "meshTexture");

        gl.uniform4fv_with_f32_array(clip_plane_uni.as_ref(), &mut opts.clip_plane.clone()[..]);

        //let mut view = if opts.flip_camera_y {
        //    state.camera().view_flipped_y()
        //} else {
        //    state.camera().view()
        //};
        //gl.uniform_matrix4fv_with_f32_array(view_uni.as_ref(), false, &mut view);

        //let camera_pos = state.camera().get_eye_pos();
        //let mut camera_pos = [camera_pos.x, camera_pos.y, camera_pos.z];
        //gl.uniform3fv_with_f32_array(camera_pos_uni.as_ref(), &mut camera_pos);

        //let mut perspective = state.camera().projection();
        //gl.uniform_matrix4fv_with_f32_array(perspective_uni.as_ref(), false, &mut perspective);

        let model = Isometry3::new(opts.pos, opts.rot);
        let mut model_array = [0.; 16];
        model_array.copy_from_slice(model.to_homogeneous().as_slice());
        gl.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut model_array);

        gl.uniform1i(mesh_texture_uni.as_ref(), TextureUnit::Stone.texture_unit());

        let block_index = shader.get_uniform_block_index(gl, "Camera");
        camera.bind_base(gl, shader, block_index, 2);

        gl.draw_elements_with_i32(
            GL::TRIANGLES,
            mesh.indices().unwrap().count() as i32,
            buffer.tri_size,
            0,
        );
    }
}
