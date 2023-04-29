use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

pub struct Tex {
    texture: Option<WebGlTexture>,
}
#[derive(Clone, Copy)]
pub struct TexUnit {
    unit: u32,
}
///webgl must support at least 32, so get_parameter will always return more than this
static mut MAX_UNITS: u32 = 31;

impl TexUnit {
    pub fn new(gl: &GL, unit: u32) -> TexUnit {
        // this app is single threaded - the chance of this failing is 0
        unsafe {
            if MAX_UNITS == 31 && unit >= MAX_UNITS {
                MAX_UNITS = gl
                    .get_parameter(GL::MAX_COMBINED_TEXTURE_IMAGE_UNITS)
                    .expect("Failed to get max combined texture image units")
                    .as_f64()
                    .unwrap() as u32;
            }

            if unit >= MAX_UNITS {
                panic!("Out of bounds texture units");
            } else {
                TexUnit { unit }
            }
        }
    }

    pub fn unit(&self) -> u32 {
        self.unit
    }
}

#[derive(Clone, Copy)]
pub enum TexFilter {
    Nearest = GL::NEAREST as isize,
    Linear = GL::LINEAR as isize,
}

impl Tex {
    pub fn bind_at(&self, gl: &GL, unit: &TexUnit) {
        gl.active_texture(GL::TEXTURE0 + unit.unit());
        gl.bind_texture(GL::TEXTURE_2D, self.texture.as_ref());
    }

    fn create_texture(gl: &GL) -> Option<WebGlTexture> {
        gl.active_texture(GL::TEXTURE31);

        let texture = gl.create_texture();

        gl.bind_texture(GL::TEXTURE_2D, texture.as_ref());

        texture
    }

    pub fn new_from_img(gl: &GL, image: std::rc::Rc<std::cell::RefCell<HtmlImageElement>>) -> Tex {
        let texture = Self::create_texture(gl);

        //    gl.pixel_storei(GL::UNPACK_FLIP_Y_WEBGL, 1);

        gl.tex_parameteri(
            GL::TEXTURE_2D,
            GL::TEXTURE_MIN_FILTER,
            GL::NEAREST_MIPMAP_LINEAR as i32,
        );
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);

        gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            GL::RGBA,
            GL::UNSIGNED_BYTE,
            &image.borrow(),
        )
        .expect("Texture image 2d");

        gl.generate_mipmap(GL::TEXTURE_2D);

        Tex { texture }
    }

    pub fn new_error(gl: &GL) -> Tex {
        let texture = Self::create_texture(gl);

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);

        // black, magenta, magenta, black
        let data_array = super::to_array_buffer_view(&[
            0, 255, 0, 255, 255, 0, 255, 255, 255, 0, 255, 255, 0, 255, 0, 255,
        ]);

        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            2,
            2,
            0,
            GL::RGBA as u32,
            GL::UNSIGNED_BYTE,
            Some(&data_array),
        )
        .expect("Failed making error texture");

        Tex { texture }
    }

    pub fn new_color(gl: &GL, width: i32, height: i32, filter: TexFilter) -> Result<Tex, JsValue> {
        let texture = Self::create_texture(gl);

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, filter as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, filter as i32);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D,
            0,
            GL::RGBA as i32,
            width,
            height,
            0,
            GL::RGBA as u32,
            GL::UNSIGNED_BYTE,
            None,
        )?;

        Ok(Tex { texture })
    }
    pub fn new_depth(gl: &GL, width: i32, height: i32) -> Result<Tex, JsValue> {
        let depth_texture = Self::create_texture(gl);

        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
        gl.tex_parameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            GL::TEXTURE_2D,
            0,
            GL::DEPTH_COMPONENT16 as i32, // Internal format
            width,
            height,
            0,
            GL::DEPTH_COMPONENT as u32, // Format
            GL::UNSIGNED_SHORT,
            None,
        )?;

        Ok(Tex {
            texture: depth_texture,
        })
    }

    pub fn texture(&self) -> Option<&WebGlTexture> {
        self.texture.as_ref()
    }
}
