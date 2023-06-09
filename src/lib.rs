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

// #![deny(missing_docs)]
extern crate num;
#[macro_use]
extern crate num_derive;
extern crate wasm_bindgen;

pub(crate) use self::app::*;
use self::canvas::*;
use self::render::*;
use app::keyboard::KeyCode;
use console_error_panic_hook;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::num::FromPrimitive;

mod app;
mod canvas;
mod fetch;

/// Used to run the application from the web
#[wasm_bindgen]
pub struct WebClient {
    app: Rc<App>,
    gl: Rc<WebGl2RenderingContext>,
    canvas: HtmlCanvasElement,
    renderer: WebRenderer,
}
#[wasm_bindgen]
impl WebClient {
    /// Create a new web client
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        wasm_logger::init(wasm_logger::Config::default());

        let app = Rc::new(App::new());

        let (gl, canvas) = create_webgl_context(Rc::clone(&app)).unwrap();

        let renderer = WebRenderer::new(&gl);

        WebClient {
            app,
            gl: Rc::new(gl),
            canvas,
            renderer,
        }
    }

    /// Start our WebGL Water application. `index.html` will call this function in order
    /// to begin rendering.
    pub fn start(&self) -> Result<(), JsValue> {
        //Load default assets, mainly error texture and possibly primitives
        self.app.assets.borrow_mut().load(&self.gl);
        Ok(())
    }

    /// Update our simulation
    pub fn update(&self, dt: f32) {
        let mut store = self.app.store.borrow_mut();
        store.msg(&Msg::AdvanceClock(dt));

        match self.app.control.try_borrow_mut() {
            Ok(mut c) => {
                //TODO: fix updates
                // for e in &mut store.state.entities {
                //     if let Entity::EntMesh(m) = &**e {
                //         let f = { m.borrow().update.as_ref().map(|f| f.clone()) };

                //         if let Some(f) = f {
                //             c.run_func(&f, m.clone());
                //         }
                //     }
                // }
            }
            Err(_) => (),
        }
    }
    /// To be called on the 'keydown' event
    pub fn on_key_down(&mut self, key_code: u32) {
        if let Some(k) = KeyCode::from_u32(key_code) {
            self.app.store.borrow_mut().msg(&Msg::KeyDown(k));
        }
    }
    /// To be called on the 'keyup' event
    pub fn on_key_up(&mut self, key_code: u32) {
        if let Some(k) = KeyCode::from_u32(key_code) {
            self.app.store.borrow_mut().msg(&Msg::KeyUp(k));
        }
    }

    /// Update our simulation
    pub async fn restart(&self, onload: String) -> String {
        let s = match self.app.control.try_borrow_mut() {
            Ok(mut c) => match c.lua_msg(
                &LuaMsg::Load(onload),
                self.app.store.clone(),
                self.app.assets.clone(),
            ) {
                Ok(()) => "".to_owned(),
                Err(str) => str,
            },
            Err(e) => e.to_string(),
        };

        //load requirements - meshes, textures, etc

        Assets::load_requirements(self.app.assets.clone(), self.gl.clone()).await;

        self.app.assets.borrow_mut().require_mesh_textures();

        //Load materials from models

        Assets::load_requirements(self.app.assets.clone(), self.gl.clone()).await;

        s
    }

    /// Render the scene. `index.html` will call this once every requestAnimationFrame
    pub fn render(&self) {
        update_display(&self.canvas, &mut self.app.store.borrow_mut().state);

        self.renderer.render(
            &self.gl,
            &self.app.store.borrow().state,
            &self.app.assets.borrow(),
        );
    }
}
