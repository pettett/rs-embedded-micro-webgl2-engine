use crate::app::store::Mesh;

use super::mark_requirements_trait::MarkRequirements;

impl MarkRequirements for Mesh {
    fn mark_requirements(&self, assets: &mut super::Assets) {
        assets.require_gltf(self.name.clone());
        assets.require_texture(self.tex.clone());
    }
}
