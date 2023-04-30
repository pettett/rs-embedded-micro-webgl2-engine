use web_sys::WebGl2RenderingContext;

use crate::app::{
    from_rhai::FromRhai,
    render::rgl::{shader::Shader, texture::TexUnit},
    Assets,
};
#[derive(Copy, Clone, Debug)]
pub enum Uniform {
    Float(f32),
    Int(i32),
    Tex(usize),
}

#[derive(Clone, Debug)]
pub struct Mat {
    uniforms: Vec<(String, Uniform)>,
}
impl Default for Mat {
    fn default() -> Self {
        Self {
            uniforms: Vec::new(),
        }
    }
}

impl Mat {
    pub const fn new() -> Self {
        Self {
            uniforms: Vec::new(),
        }
    }
    pub fn filled(uniforms: Vec<(String, Uniform)>) -> Self {
        Self { uniforms }
    }

    pub fn uniform(&self, gl: &WebGl2RenderingContext, shader: &Shader, assets: &Assets) {
        let mut i = 10;
        for (name, uniform) in &self.uniforms {
            let loc = shader.get_uniform_location(gl, name);

            match uniform {
                Uniform::Float(v) => gl.uniform1f(loc.as_ref(), *v),
                Uniform::Int(v) => gl.uniform1i(loc.as_ref(), *v),
                Uniform::Tex(v) => {
                    let u = TexUnit::new(gl, i);
                    assets.get_tex(*v).bind_at(gl, &u);

                    gl.uniform1i(loc.as_ref(), u.unit() as i32);
                    i += 1
                }
            }
        }
    }
}

impl FromRhai for Mat {
    fn try_from_rhai(map: rhai::Map, assets: &mut crate::app::Assets) -> Result<Self, &'static str>
    where
        Self: Sized,
    {
        let mut uniforms = Vec::new();

        for (k, v) in map.iter() {
            if v.is_variant() {
                uniforms.push((k.to_owned().to_string(), v.clone().cast()));
            }
        }
        Ok(Mat { uniforms })
    }
}
