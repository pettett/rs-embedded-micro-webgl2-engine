// use crate::render::mesh::MeshRenderOpts;
// use crate::render::mesh::NonSkinnedMesh;
// use crate::render::mesh::SkinnedMesh;
use crate::render::rgl::shader::ShaderKind;
use crate::render::Render;
use crate::render::WebRenderer;
use crate::Assets;
use crate::State;
use nalgebra::ArrayStorage;
use nalgebra::Vector3;
use web_sys::WebGl2RenderingContext as GL;

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
    pub(in crate::render) fn render_meshes(
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

        let (_, no_skin) = (ShaderKind::SkinnedMesh, ShaderKind::NonSkinnedMesh);

        let non_skinned_shader = self.shader_sys.get_shader(&no_skin).unwrap();

        let mesh_mat = MatAlbedo {
            shader: non_skinned_shader.clone(),
            //    tex: assets.get_tex("assets/textures/stone-texture.png"),
            tex: assets.get_tex("random_texture.png"),
        };

        self.shader_sys.use_program(gl, ShaderKind::NonSkinnedMesh);

        mesh_mat.bind_uniforms(gl, camera, state);

        // Render Terrain

        for entity in &state.entities {
            if let crate::app::Entity::EntMesh(mesh) = &**entity {
                let ent = mesh.borrow();

                let mut mesh_opts = MeshRenderOpts {
                    pos: ent.position,
                    rot: ent.rotation,
                    clip_plane,
                    flip_camera_y,
                };

                if let Some(doc) = assets.get_gltf(&ent.name) {
                    for node in doc.doc.nodes() {
                        if let Some(m) = node.mesh() {
                            //get primitives
                            for p in m.primitives() {
                                match node.transform() {
                                    gltf::scene::Transform::Matrix { matrix: _ } => todo!(),
                                    gltf::scene::Transform::Decomposed {
                                        translation,
                                        rotation: _,
                                        scale: _,
                                    } => {
                                        mesh_opts.pos = ent.position
                                            + Vector3::from_array_storage(ArrayStorage([
                                                translation,
                                            ]));
                                    }
                                }

                                let meshdata = NonSkinnedGltfMesh {
                                    mesh: &p,
                                    buffers: &doc.buffers,
                                    shader: non_skinned_shader.clone(),
                                    opts: &mesh_opts,
                                };

                                //    web_sys::console::log_1(&p.index().into());
                                //web_sys::console::log_1(&"Rendering mesh".into());

                                let b = self.prepare_for_render(
                                    gl,
                                    &meshdata,
                                    &format!("{}{}{}", &ent.name, m.index(), p.index()),
                                    state,
                                );

                                meshdata.render(gl, &b, camera, state);
                            }
                        }
                    }
                }
            }
        }

        // let non_skinned_shader = self.shader_sys.get_shader(&no_skin).unwrap();
        // self.shader_sys.use_program(gl, ShaderKind::NonSkinnedMesh);

        // let mesh_opts = MeshRenderOpts {
        //     pos: (0., 0., 0.),
        //     clip_plane,
        //     flip_camera_y,
        // };

        // let mesh_name = "Terrain";
        // let terrain = NonSkinnedMesh {
        //     mesh: assets.get_mesh(mesh_name).expect("Terrain mesh"),
        //     shader: non_skinned_shader,
        //     opts: &mesh_opts,
        // };

        // self.prepare_for_render(gl, &terrain, mesh_name);
        // terrain.render(gl, state);

        // Render Bird

        // let skinned_shader = self.shader_sys.get_shader(&skin).unwrap();
        // self.shader_sys.use_program(gl, ShaderKind::SkinnedMesh);

        // let bird_traveled = (state.clock() / 1000.0) * BIRD_SPEED;
        // let z = BIRD_START_Z + (bird_traveled % (BIRD_END_Z - BIRD_START_Z));

        // let mesh_opts = MeshRenderOpts {
        //     pos: (0., 6., z),
        //     clip_plane,
        //     flip_camera_y,
        // };

        // let mesh_name = "Bird";
        // let armature_name = "Armature.001";
        // let bird = SkinnedMesh {
        //     mesh: assets.get_mesh(mesh_name).expect("Bird mesh"),
        //     armature: assets.get_armature(armature_name).expect("Bird armature"),
        //     shader: skinned_shader,
        //     opts: &mesh_opts,
        // };

        // self.prepare_for_render(gl, &bird, mesh_name);
        // bird.render(gl, state);
    }
}
