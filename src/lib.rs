//! An example of how to render water using WebGL + Rust + WebAssembly.
//!
//! We'll try to heavily over comment the code so that it's more accessible to those that
//! are less familiar with the techniques that are used.
//!
//! In a real application you'd split things up into different modules and files,
//! but I tend to prefer tutorials that are all in one file that you can scroll up and down in
//! and soak up what you see vs. needing to hop around different files.
//!
//! If you have any questions or comments feel free to open an issue on GitHub!
//!
//! https://github.com/chinedufn/webgl-water-tutorial
//!
//! Heavily inspired by this @thinmatrix tutorial:
//!   - https://www.youtube.com/watch?v=HusvGeEDU_U&list=PLRIWtICgwaX23jiqVByUs0bqhnalNTNZh

#![deny(missing_docs)]

extern crate wasm_bindgen;
pub(crate) use self::app::*;
use self::canvas::*;
use self::render::*;
use crate::load_texture_img::load_texture_image;
use console_error_panic_hook;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;

mod app;
mod canvas;
mod fetch;
mod load_texture_img;
mod render;

/// Used to run the application from the web
#[wasm_bindgen]
pub struct WebClient {
    app: Rc<App>,
    gl: Rc<WebGl2RenderingContext>,
    renderer: WebRenderer,
}
#[wasm_bindgen]
impl WebClient {
    /// Create a new web client
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        let app = Rc::new(App::new());

        let gl = Rc::new(create_webgl_context(Rc::clone(&app)).unwrap());

        let renderer = WebRenderer::new(&gl);

        WebClient { app, gl, renderer }
    }

    /// Start our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn start(&self) -> Result<(), JsValue> {
        let gl = &self.gl;

        load_texture_image(Rc::clone(gl), "assets/textures/dudvmap.png", TextureUnit::Dudv);
        load_texture_image(Rc::clone(gl), "assets/textures//normalmap.png", TextureUnit::NormalMap);
        load_texture_image(Rc::clone(gl), "assets/textures//stone-texture.png", TextureUnit::Stone);

        Ok(())
    }

    /// Update our simulation
    pub fn update(&self, dt: f32) {
        let mut store = self.app.store.borrow_mut();
        store.msg(&Msg::AdvanceClock(dt));

        match self.app.control.try_borrow_mut() {
            Ok(mut c) => {
                for e in &mut store.state.entities {
                    if let Entity::EntMesh(m) = &**e {
                        let f = { m.borrow().update.as_ref().map(|f| f.clone()) };

                        if let Some(f) = f {
                            c.run_func(&f, m.clone())
                        }
                    }
                }
            }
            Err(_) => (),
        }
    }

    /// Update our simulation
    pub async fn restart(&self, onload: String) -> String {
        match self.app.control.try_borrow_mut() {
            Ok(mut c) => match c
                .lua_msg(
                    &LuaMsg::Load(onload),
                    self.app.store.clone(),
                    self.app.assets.clone(),
                )
                .await
            {
                Ok(()) => "".to_owned(),
                Err(str) => str,
            },
            Err(e) => e.to_string(),
        }
    }

    /// Render the scene. `index.html` will call this once every requestAnimationFrame
    pub fn render(&self) {
        self.renderer.render(
            &self.gl,
            &self.app.store.borrow().state,
            &self.app.assets.borrow(),
        );
    }
    /// Load a new gltf mesh that can be used by entities
    pub fn load_mesh(&self, mesh_name: String, gltf: &[u8]) -> Result<(), String> {
        self.app
            .assets
            .try_borrow_mut()
            .unwrap()
            .load_gltf(mesh_name, gltf)
            .map_err(|e| e.to_string())
    }
}
