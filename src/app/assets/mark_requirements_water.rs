use crate::app::store::water::Water;

use super::mark_requirements_trait::MarkRequirements;

impl MarkRequirements for Water {
    fn mark_requirements(&self, assets: &mut super::Assets) {
        assets.require_texture("assets/textures/dudvmap.png".to_owned());
        assets.require_texture("assets/textures/normalmap.png".to_owned());
    }
}
