#[derive(Debug, Clone)]
pub struct Water {
    pub dudv: usize,
    pub normal: usize,
    pub reflectivity: f32,
    pub fresnel_strength: f32,
    pub wave_speed: f32,
    pub use_refraction: bool,
    pub use_reflection: bool,
}
