use super::Assets;

pub trait MarkRequirements {
    fn mark_requirements(&self, assets: &mut Assets);
}
