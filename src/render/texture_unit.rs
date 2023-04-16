use web_sys::WebGl2RenderingContext as GL;

#[derive(Clone, Copy)]
pub enum TextureUnit {
    Refraction = 0,
    Reflection = 1,
    Dudv = 2,
    NormalMap = 3,
    RefractionDepth = 4,
    Stone = 5,
}

impl TextureUnit {
    /// gl.TEXTURE1, gl.TEXTURE2 ... etc. Useful for `gl.active_texture`
    #[allow(non_snake_case)]
    pub fn TEXTURE_N(&self) -> u32 {
        GL::TEXTURE0 + (self.texture_unit() as u32)
    }

    /// 0, 1, 2, ... etc. Useful for `gl.uniform1i` calls
    pub fn texture_unit(&self) -> i32 {
        *self as i32
    }
}
