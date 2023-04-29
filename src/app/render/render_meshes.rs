use crate::app::Mat;
// use crate::app::render::mesh::MeshRenderOpts;
// use crate::app::render::mesh::NonSkinnedMesh;
// use crate::app::render::mesh::SkinnedMesh;
use crate::app::render::rgl::shader::ShaderKind;
use crate::app::render::Render;
use crate::app::render::WebRenderer;
use crate::Assets;
use crate::State;
use nalgebra::ArrayStorage;
use nalgebra::Vector3;
use web_sys::WebGl2RenderingContext as GL;

use super::cube::Cube;
use super::material::MatAlbedo;
use super::material::Material;
use super::rgl::uniform_buffer::UniformBuffer;
use super::CameraData;
use super::MeshRenderOpts;
use super::NonSkinnedGltfMesh;

// static BIRD_SPEED: f32 = 3.5;
// static BIRD_START_Z: f32 = -30.0;
// static BIRD_END_Z: f32 = 30.0;

impl WebRenderer {
    pub(in crate::app::render) fn render_meshes(
        &self,
        gl: &GL,
        state: &State,
        assets: &Assets,
        camera: &UniformBuffer<CameraData>,
        clip_plane: [f32; 4],
        flip_camera_y: bool,
    ) {
        if !state.show_scenery() {
            return;
        }
        let non_skinned_shader = self
            .shader_sys
            .get_shader(&ShaderKind::NonSkinnedMesh)
            .unwrap();

        self.shader_sys.use_program(gl, ShaderKind::NonSkinnedMesh);
        let mut i = 0;
        for entity in &state.entities {
            let e = entity.borrow();
            let b =
                self.prepare_for_render(gl, &**e, non_skinned_shader, &format!("{}", &i), state);

            e.render(gl, &b, non_skinned_shader, &self, camera, state, assets);
            i += 1;
        }

        // let non_skinned_shader = self
        //     .shader_sys
        //     .get_shader(&ShaderKind::NonSkinnedMesh)
        //     .unwrap();

        // self.shader_sys.use_program(gl, ShaderKind::NonSkinnedMesh);

        // // Render Meshes

        // let mut gizmos: Vec<(Vector3<f32>, Vector3<f32>)> = Vec::new();

        // for entity in &state.entities {
        //     if let crate::app::Entity::EntMesh(mesh) = &**entity {
        //         let ent = mesh.borrow();

        //         let mut mesh_opts = MeshRenderOpts {
        //             pos: ent.position,
        //             scale: ent.scale,
        //             rot: ent.rotation,
        //             clip_plane,
        //             flip_camera_y,
        //         };

        //         if let Some(doc) = assets.get_gltf(ent.mesh) {
        //             for node in doc.doc.nodes() {
        //                 match node.transform() {
        //                     gltf::scene::Transform::Matrix { matrix: _ } => todo!(),
        //                     gltf::scene::Transform::Decomposed {
        //                         translation,
        //                         rotation: _,
        //                         scale,
        //                     } => {
        //                         let s = Vector3::from_array_storage(ArrayStorage([scale]));

        //                         mesh_opts.pos = ent.position
        //                             + Vector3::from_array_storage(ArrayStorage([translation]));

        //                         mesh_opts.scale = mesh_opts.scale.component_mul(&s);
        //                     }
        //                 }

        //                 if let Some(m) = node.mesh() {
        //                     //get primitives
        //                     for p in m.primitives() {
        //                         let meshdata = NonSkinnedGltfMesh {
        //                             mesh: &p,
        //                             buffers: &doc.buffers,
        //                             opts: &mesh_opts,
        //                         };

        //                         let bounds = p.bounding_box();
        //                         let min = Vector3::from_data(ArrayStorage([bounds.min]));
        //                         let max = Vector3::from_data(ArrayStorage([bounds.max]));

        //                         let pos = (min + max) * 0.5f32;
        //                         let extents = (max - pos).component_mul(&mesh_opts.scale);

        //                         gizmos.push((
        //                             pos + mesh_opts.pos + Vector3::new(0.0, extents.y, 0.0),
        //                             extents,
        //                         ));

        //                         // if let Uri { uri, .. } = p
        //                         //     .material()
        //                         //     .pbr_metallic_roughness()
        //                         //     .base_color_texture()
        //                         //     .unwrap()
        //                         //     .texture()
        //                         //     .source()
        //                         //     .source()
        //                         // {}

        //                         let mat =
        //                             match p.material().index().map(|i| *assets.get_material(i)) {
        //                                 Some(Some(m)) => m,
        //                                 _ => ent.mat,
        //                             };

        //                         //    log::info!("{}", mat.normal);

        //                         let mesh_mat = MatAlbedo {
        //                             shader: non_skinned_shader.clone(),
        //                             tex: assets.get_tex(mat.tex),
        //                             normal: assets.get_tex(mat.normal),
        //                         };

        //                         mesh_mat.bind_uniforms(gl, camera, state);

        //                         //    web_sys::console::log_1(&p.index().into());
        //                         //web_sys::console::log_1(&"Rendering mesh".into());

        //                         let b = self.prepare_for_render(
        //                             gl,
        //                             &meshdata,
        //                             non_skinned_shader,
        //                             &format!("{}{}{}", &ent.mesh, m.index(), p.index()),
        //                             state,
        //                         );

        //                         meshdata.render(gl, &b, non_skinned_shader, &self, camera, state);
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
        // let wireframe_shader = self.shader_sys.get_shader(&ShaderKind::WireFrame).unwrap();

        // self.shader_sys.use_program(gl, ShaderKind::WireFrame);
        // for (pos, extents) in gizmos {
        //     let b = Cube::new(pos, extents);
        //     //    log::info!("Gizmo at p: {} e: {}", pos, extents);
        //     let buff = self.prepare_for_render(gl, &b, wireframe_shader, "gizmo", state);

        //     b.render(gl, &buff, wireframe_shader, &self, camera, state);
        // }
    }
}
