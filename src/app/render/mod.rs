use self::material::MatWater;
use self::material::Material;
pub(self) use self::mesh::*;
pub(self) use self::render_trait::*;
use self::rgl::framebuffer::*;
use self::rgl::texture::TexUnit;
use self::rgl::uniform_buffer::UniformBuffer;
use self::rgl::Framebuffer;
pub use self::texture_unit::*;
use crate::app::store::water::Water;
use crate::app::Assets;
use crate::app::State;
//use crate::canvas::{CANVAS_HEIGHT, CANVAS_WIDTH};
use crate::app::render::rgl::shader::ShaderKind;
use crate::app::render::rgl::shader::ShaderSystem;
use crate::app::render::textured_quad::TexturedQuad;
use js_sys::WebAssembly;
use nalgebra::Matrix4;
use nalgebra::Point4;
use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

pub static WATER_TILE_Y_POS: f32 = 0.0;

pub mod material;
pub mod mesh;
pub mod render_meshes;
pub mod render_trait;
pub mod rgl;
pub mod texture_unit;
use rgl::vao::Vao;

struct VaoExtension {
    vaos: RefCell<HashMap<String, (Vao, BufferedMesh)>>,
}

/// Mirrors the glsl:
///```
///layout(std140) uniform MatrixBlock
///{
///		mat4 projection;
///		mat4 view;
///		vec4 pos;
///} camera;
///```
pub struct CameraData {
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
    pos: Point4<f32>, // vec4 for padding reasons
}

pub struct WebRenderer {
    pub shader_sys: ShaderSystem,
    //    #[allow(unused)]
    //    depth_texture_ext: Option<js_sys::Object>,
    refraction_framebuffer: std::rc::Rc<Framebuffer>,
    reflection_framebuffer: std::rc::Rc<Framebuffer>,
    vao_ext: VaoExtension,
    camera_buffer: UniformBuffer<CameraData>,
    flipped_y_camera_buffer: UniformBuffer<CameraData>,
}

impl WebRenderer {
    pub fn new(gl: &WebGl2RenderingContext) -> WebRenderer {
        let shader_sys = ShaderSystem::new(&gl);

        //let depth_texture_ext = gl
        //    .get_extension("WEBGL_depth_texture")
        //    .expect("Depth texture extension");

        let vao_ext = VaoExtension {
            vaos: RefCell::new(HashMap::new()),
        };

        let refraction_framebuffer =
            std::rc::Rc::new(WebRenderer::create_refraction_framebuffer(&gl).unwrap());
        let reflection_framebuffer =
            std::rc::Rc::new(WebRenderer::create_reflection_framebuffer(&gl).unwrap());

        WebRenderer {
            //    depth_texture_ext,
            shader_sys,
            refraction_framebuffer,
            reflection_framebuffer,
            vao_ext,
            camera_buffer: UniformBuffer::new(gl),
            flipped_y_camera_buffer: UniformBuffer::new(gl),
        }
    }

    ///render
    pub fn render(&self, gl: &WebGl2RenderingContext, state: &State, assets: &Assets) {
        //web_sys::console::log_1(&"Rendering".into());
        let mut water: Option<&Water> = None;
        // for entity in &state.entities {
        //     if let crate::app::Entity::EntWater(w) = &**entity {
        //         water = Some(w)
        //     }
        // }

        gl.clear_color(0.53, 0.8, 0.98, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let p = state.camera().get_eye_pos();

        let camera = CameraData {
            view: state.camera().view_mat(),
            projection: state.camera().projection_mat().clone(),
            pos: Point4::new(p.x, p.y, p.z, 0.0),
        };

        self.camera_buffer.buffer(gl, &camera);

        let above = 1000000.0;
        // Position is positive instead of negative for.. mathematical reasons..
        let clip_plane = [0., 1., 0., above];

        if let Some(w) = &water {
            if w.use_reflection {
                let flipped_y_camera = CameraData {
                    view: state.camera().view_flipped_y_mat(),
                    projection: camera.projection.clone(),
                    pos: camera.pos.clone(),
                };

                self.flipped_y_camera_buffer.buffer(gl, &flipped_y_camera);
            }

            self.render_refraction_fbo(gl, w, &self.camera_buffer, state, assets);
            self.render_reflection_fbo(gl, w, &self.flipped_y_camera_buffer, state, assets);
            gl.viewport(
                0,
                0,
                state.display.width as i32,
                state.display.height as i32,
            );
            gl.bind_framebuffer(GL::FRAMEBUFFER, None);
        }

        if let Some(w) = water {
            self.render_water(gl, w, &self.camera_buffer, state, assets);
        }

        self.render_meshes(gl, state, assets, &self.camera_buffer, clip_plane, false);

        if let Some(w) = water {
            self.render_refraction_visual(gl, &self.camera_buffer, state, assets);
            self.render_reflection_visual(gl, &self.camera_buffer, state, assets);
        }

        //DEBUG: Display 30 loaded textures

        let u = TexUnit::new(gl, 10);
        for i in 0..30 {
            assets.get_tex(i).bind_at(gl, &u);
            self.render_visual(gl, &self.camera_buffer, state, assets, u, 70 * i as u16, 70);
        }
    }

    fn render_water(
        &self,
        gl: &WebGl2RenderingContext,
        water: &Water,
        camera: &UniformBuffer<CameraData>,
        state: &State,
        assets: &Assets,
    ) {
        let water_shader = self.shader_sys.get_shader(&ShaderKind::Water).unwrap();
        self.shader_sys.use_program(gl, ShaderKind::Water);

        let water_material = MatWater {
            shader: water_shader.clone(),
            dudv: assets.get_tex(water.dudv),
            normal_map: assets.get_tex(water.normal),
            refraction: self.refraction_framebuffer.clone(),
            reflection: self.reflection_framebuffer.clone(),
            reflectivity: water.reflectivity,
            fresnel_strength: water.fresnel_strength,
            wave_speed: water.wave_speed,
            use_refraction: water.use_refraction,
            use_reflection: water.use_refraction,
        };

        let b = self.prepare_for_render(gl, water, water_shader, "water", state);

        water_material.bind_uniforms(gl, camera, state);
        water.render(gl, &b, water_shader, &self, camera, state, assets);
    }

    fn render_refraction_fbo(
        &self,
        gl: &WebGl2RenderingContext,
        water: &Water,
        camera: &UniformBuffer<CameraData>,
        state: &State,
        assets: &Assets,
    ) {
        let framebuffer = &self.refraction_framebuffer.framebuffer;
        gl.bind_framebuffer(GL::FRAMEBUFFER, framebuffer.as_ref());

        gl.viewport(0, 0, REFRACTION_TEXTURE_WIDTH, REFRACTION_TEXTURE_HEIGHT);

        gl.clear_color(0.53, 0.8, 0.98, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        if water.use_refraction {
            let clip_plane = [0., -1., 0., WATER_TILE_Y_POS];
            self.render_meshes(gl, state, assets, camera, clip_plane, false);
        }
    }

    fn render_reflection_fbo(
        &self,
        gl: &WebGl2RenderingContext,
        water: &Water,
        camera: &UniformBuffer<CameraData>,
        state: &State,
        assets: &Assets,
    ) {
        let framebuffer = &self.reflection_framebuffer.framebuffer;
        gl.bind_framebuffer(GL::FRAMEBUFFER, framebuffer.as_ref());

        gl.viewport(0, 0, REFLECTION_TEXTURE_WIDTH, REFLECTION_TEXTURE_HEIGHT);

        gl.clear_color(0.53, 0.8, 0.98, 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        if water.use_reflection {
            let clip_plane = [0., 1., 0., -WATER_TILE_Y_POS];
            self.render_meshes(gl, state, assets, camera, clip_plane, true);
        }
    }

    fn render_visual(
        &self,
        gl: &WebGl2RenderingContext,
        camera: &UniformBuffer<CameraData>,
        state: &State,
        assets: &Assets,
        tex_unit: TexUnit,
        x: u16,
        y: u16,
    ) {
        let quad_shader = self
            .shader_sys
            .get_shader(&ShaderKind::TexturedQuad)
            .unwrap();
        self.shader_sys.use_program(gl, ShaderKind::TexturedQuad);

        let textured_quad = TexturedQuad::new(x, y, 35, 35, tex_unit, quad_shader.clone());

        let b = self.prepare_for_render(gl, &textured_quad, quad_shader, "VisualMesh", state);
        textured_quad.render(gl, &b, quad_shader, &self, camera, state, assets);
    }

    fn render_refraction_visual(
        &self,
        gl: &WebGl2RenderingContext,
        camera: &UniformBuffer<CameraData>,
        state: &State,
        assets: &Assets,
    ) {
        let quad_shader = self
            .shader_sys
            .get_shader(&ShaderKind::TexturedQuad)
            .unwrap();
        self.shader_sys.use_program(gl, ShaderKind::TexturedQuad);

        let textured_quad = TexturedQuad::new(
            0,
            75,
            75,
            75,
            TexUnit::new(gl, TextureUnit::Refraction.texture_unit() as u32),
            quad_shader.clone(),
        );

        let b = self.prepare_for_render(gl, &textured_quad, quad_shader, "RefractionVisual", state);
        textured_quad.render(gl, &b, quad_shader, &self, camera, state, assets);
    }

    fn render_reflection_visual(
        &self,
        gl: &WebGl2RenderingContext,
        camera: &UniformBuffer<CameraData>,
        state: &State,
        assets: &Assets,
    ) {
        let quad_shader = self
            .shader_sys
            .get_shader(&ShaderKind::TexturedQuad)
            .unwrap();
        self.shader_sys.use_program(gl, ShaderKind::TexturedQuad);
        let textured_quad = TexturedQuad::new(
            state.display.width as u16 - 75,
            state.display.height as u16,
            75,
            75,
            TexUnit::new(gl, TextureUnit::Reflection.texture_unit() as u32),
            quad_shader.clone(),
        );

        let b = self.prepare_for_render(gl, &textured_quad, quad_shader, "ReflectionVisual", state);
        textured_quad.render(gl, &b, quad_shader, &self, camera, state, assets);
    }

    pub fn prepare_for_render(
        &self,
        gl: &WebGl2RenderingContext,
        renderable: &(impl Render + ?Sized),
        shader: &rgl::shader::Shader,
        key: &str,
        state: &State,
    ) -> BufferedMesh {
        if self.vao_ext.vaos.borrow().get(key).is_none() {
            let vao = Vao::new(gl);
            vao.bind(gl);
            let b = renderable.buffer_attributes(gl, shader, state);
            let mut vaos = self.vao_ext.vaos.borrow_mut();

            vaos.insert(key.to_string(), (vao, b));

            return vaos.get(key).unwrap().1;
        }

        let vaos = self.vao_ext.vaos.borrow();
        let (vao, b) = vaos.get(key).unwrap();

        vao.bind(gl);
        *b
    }
}

///buffer
pub fn buffer_f32_data(gl: &GL, data: &[f32], attrib: u32, size: i32) {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

    let data_location = data.as_ptr() as u32 / 4;

    let data_array = js_sys::Float32Array::new(&memory_buffer)
        .subarray(data_location, data_location + data.len() as u32);

    let buffer = gl.create_buffer().unwrap();

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &data_array, GL::STATIC_DRAW);
    gl.vertex_attrib_pointer_with_i32(attrib, size, GL::FLOAT, false, 0, 0);
}

///buffer
pub fn buffer_sf32_data<const S: usize>(gl: &GL, data: &[[f32; S]], attrib: u32) {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

    let data_location = data.as_ptr() as u32 / 4;

    let data_array = js_sys::Float32Array::new(&memory_buffer)
        .subarray(data_location, data_location + (data.len() * S) as u32);

    let buffer = gl.create_buffer().unwrap();

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &data_array, GL::STATIC_DRAW);
    gl.vertex_attrib_pointer_with_i32(attrib, S as i32, GL::FLOAT, false, 0, 0);
}

///buffer
pub fn buffer_u8_data(gl: &GL, data: &[u8], attrib: u32, size: i32) {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

    let data_location = data.as_ptr() as u32;

    let data_array = js_sys::Uint8Array::new(&memory_buffer)
        .subarray(data_location, data_location + data.len() as u32);

    let buffer = gl.create_buffer().unwrap();

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &data_array, GL::STATIC_DRAW);
    gl.vertex_attrib_pointer_with_i32(attrib, size, GL::UNSIGNED_BYTE, false, 0, 0);
}

///buffer
pub fn buffer_u16_indices(gl: &GL, indices: &[u16]) {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

    let indices_location = indices.as_ptr() as u32 / 2;
    let indices_array = js_sys::Uint16Array::new(&memory_buffer)
        .subarray(indices_location, indices_location + indices.len() as u32);

    let index_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    gl.buffer_data_with_array_buffer_view(
        GL::ELEMENT_ARRAY_BUFFER,
        &indices_array,
        GL::STATIC_DRAW,
    );
}
///buffer
pub fn buffer_u32_indices(gl: &GL, indices: &[u32]) {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

    let indices_location = indices.as_ptr() as u32 / 2;
    let indices_array = js_sys::Uint32Array::new(&memory_buffer)
        .subarray(indices_location, indices_location + indices.len() as u32);

    let index_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    gl.buffer_data_with_array_buffer_view(
        GL::ELEMENT_ARRAY_BUFFER,
        &indices_array,
        GL::STATIC_DRAW,
    );
}