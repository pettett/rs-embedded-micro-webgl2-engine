use crate::app::render::mesh::cube::Cube;
use crate::app::render::mesh::MeshRenderOpts;
use crate::app::render::mesh::NonSkinnedGltfMesh;
use crate::app::render::render_trait::Render;
use crate::app::render::rgl::shader::ShaderKind;
use crate::app::render::rgl::uniform_buffer::UniformBuffer;
use crate::app::render::RenderStage;
use crate::app::render::{CameraData, WebRenderer};
use crate::app::Assets;
use crate::app::Control;
use crate::app::State;
use nalgebra;
use nalgebra::ArrayStorage;
use nalgebra::Vector3;
use web_sys::WebGl2RenderingContext as GL;

use super::entity::Entity;
#[derive(Debug, Clone)]
pub struct Mesh {
    pub mesh: usize,
    pub mat: usize,

    pub position: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub update: Option<String>,
}

impl Entity for Mesh {
    fn should_render(&self, stage: &RenderStage) -> bool {
        *stage != RenderStage::Water
    }

    fn render(
        &self,
        gl: &GL,
        renderer: &WebRenderer,
        camera: &UniformBuffer<CameraData>,
        clip_plane: [f32; 4],
        stage: RenderStage,
        state: &State,
        assets: &Assets,
    ) {
        //log::info!("Rendering mesh");

        // Render Meshes
        let non_skinned_shader = renderer
            .shader_sys
            .get_shader(&ShaderKind::NonSkinnedMesh)
            .unwrap();
        renderer
            .shader_sys
            .use_program(gl, ShaderKind::NonSkinnedMesh);

        let mut gizmos: Vec<(Vector3<f32>, Vector3<f32>)> = Vec::new();

        let mut mesh_opts = MeshRenderOpts {
            pos: self.position,
            scale: self.scale,
            rot: self.rotation,
            clip_plane,
            flip_camera_y: false,
        };

        if let Some(doc) = assets.get_gltf(self.mesh) {
            for node in doc.doc.nodes() {
                match node.transform() {
                    gltf::scene::Transform::Matrix { matrix: _ } => todo!(),
                    gltf::scene::Transform::Decomposed {
                        translation,
                        rotation: _,
                        scale,
                    } => {
                        let s = Vector3::from_array_storage(ArrayStorage([scale]));

                        mesh_opts.pos = self.position
                            + Vector3::from_array_storage(ArrayStorage([translation]));

                        mesh_opts.scale = mesh_opts.scale.component_mul(&s);
                    }
                }

                if let Some(m) = node.mesh() {
                    //get primitives
                    for p in m.primitives() {
                        let meshdata = NonSkinnedGltfMesh {
                            mesh: &p,
                            buffers: &doc.buffers,
                            opts: &mesh_opts,
                        };

                        let bounds = p.bounding_box();
                        let min = Vector3::from_data(ArrayStorage([bounds.min]));
                        let max = Vector3::from_data(ArrayStorage([bounds.max]));

                        let pos = (min + max) * 0.5f32;
                        let extents = (max - pos).component_mul(&mesh_opts.scale);

                        gizmos.push((
                            pos + mesh_opts.pos + Vector3::new(0.0, extents.y, 0.0),
                            extents,
                        ));

                        // if let Uri { uri, .. } = p
                        //     .material()
                        //     .pbr_metallic_roughness()
                        //     .base_color_texture()
                        //     .unwrap()
                        //     .texture()
                        //     .source()
                        //     .source()
                        // {}

                        if let Some(mat) = assets.get_material(match p.material().index() {
                            Some(i) => i,
                            None => self.mat,
                        }) {
                            mat.uniform(gl, &non_skinned_shader, assets);
                        };

                        //    log::info!("{}", mat.normal);

                        // let mesh_mat = MatAlbedo {
                        //     shader: non_skinned_shader.clone(),
                        //     tex: assets.get_tex(mat.tex),
                        //     normal: assets.get_tex(mat.normal),
                        // };

                        let block_index = non_skinned_shader.get_uniform_block_index(gl, "Camera");
                        camera.bind_base(gl, &non_skinned_shader, block_index, 2);

                        //mesh_mat.bind_uniforms(gl, camera, state);

                        //    web_sys::console::log_1(&p.index().into());
                        //web_sys::console::log_1(&"Rendering mesh".into());

                        let b = renderer.prepare_for_render(
                            gl,
                            &meshdata,
                            non_skinned_shader,
                            &format!("{}{}{}", &self.mesh, m.index(), p.index()),
                            state,
                        );

                        meshdata.render(gl, &b, non_skinned_shader, renderer, camera, state);
                    }
                }
            }
        }
        if stage == RenderStage::Opaques {
            let wireframe_shader = renderer
                .shader_sys
                .get_shader(&ShaderKind::WireFrame)
                .unwrap();

            renderer.shader_sys.use_program(gl, ShaderKind::WireFrame);
            for (pos, extents) in gizmos {
                let b = Cube::new(pos, extents);
                //    log::info!("Gizmo at p: {} e: {}", pos, extents);
                let buff = renderer.prepare_for_render(gl, &b, wireframe_shader, "gizmo", state);

                b.render(gl, &buff, wireframe_shader, renderer, camera, state)
            }
        }
    }

    fn update(&mut self, control: &Control) {
        if let Some(f) = &self.update {
            //    control.run_func(&f, m.clone());
        }
    }
}
