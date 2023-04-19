mod gltf_mesh;
// mod non_skinned_mesh;
// mod skinned_mesh;

use nalgebra::Vector3;

pub mod textured_quad;
pub mod water_tile;

pub use self::gltf_mesh::*;
// pub use self::non_skinned_mesh::*;
// pub use self::skinned_mesh::*;

pub struct MeshRenderOpts {
    pub pos: Vector3<f32>,
    pub rot: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub clip_plane: [f32; 4],
    pub flip_camera_y: bool,
}
