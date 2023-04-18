use crate::app::Assets;

pub trait FromRhai {
    fn try_from_rhai(map: rhai::Map, assets: &mut Assets) -> Result<Self, &'static str>
    where
        Self: Sized;
}
