pub mod display;
pub mod entity;
pub mod keyboard;
pub mod mesh;
mod mouse;

use nalgebra::Vector3;

use crate::app::render::render_trait::Render;

use self::display::Display;
use self::entity::Entity;
use self::keyboard::KeyCode;
use self::keyboard::Keyboard;
use self::mouse::*;

mod camera;
use self::camera::*;
pub use self::mesh::Mesh;

pub mod water;

use super::Mat;

pub struct Store {
    // information about game state
    pub state: State,
}

impl Store {
    pub fn new() -> Store {
        Store {
            state: State::new(),
        }
    }

    pub fn msg(&mut self, msg: &Msg) {
        self.state.msg(msg)
    }
}

pub struct State {
    clock: f32,
    next_log: f32,
    dt_rolling: f32,
    pub display: super::display::Display,
    camera: Camera,
    keyboard: Keyboard,
    mouse: Mouse,
    show_scenery: bool,
    pub entities: Vec<std::rc::Rc<std::cell::RefCell<dyn Entity>>>,
}

impl State {
    fn new() -> State {
        State {
            /// Time elapsed since the application started, in milliseconds
            clock: 0.,
            next_log: 0.,
            dt_rolling: 0.,
            camera: Camera::new(),
            keyboard: Keyboard::default(),
            mouse: Mouse::default(),
            display: Display {
                width: 1,
                height: 1,
            },
            show_scenery: true,
            entities: vec![],
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
    /// The current time in milliseconds
    pub fn clock(&self) -> f32 {
        self.clock
    }

    pub fn show_scenery(&self) -> bool {
        self.show_scenery
    }

    pub fn msg(&mut self, msg: &Msg) {
        match msg {
            Msg::AdvanceClock(dt) => {
                self.clock += dt;

                self.camera.update(*dt, &self.keyboard);

                //exponential falloff rolling average
                self.dt_rolling = self.dt_rolling * 0.8 + dt * 0.2;

                if self.clock > self.next_log {
                    log::trace!("{:.3}", 1000.0 / self.dt_rolling);
                    self.next_log += 1000.0;
                }
            }
            Msg::MouseDown(x, y) => {
                self.mouse.set_pressed(true);
                self.mouse.set_pos(*x, *y);
            }
            Msg::MouseUp => {
                self.mouse.set_pressed(false);
            }
            Msg::MouseMove(x, y) => {
                if !self.mouse.get_pressed() {
                    return;
                }

                let (old_x, old_y) = self.mouse.get_pos();

                let x_delta = old_x as i32 - x;
                let y_delta = y - old_y as i32;

                self.camera.orbit_left_right(x_delta as f32 / 50.0);
                self.camera.orbit_up_down(y_delta as f32 / 50.0);

                self.mouse.set_pos(*x, *y);
            }
            Msg::Zoom(zoom) => {
                self.camera.zoom(*zoom);
            }
            Msg::KeyDown(key_code) => self.keyboard.set_pressed(*key_code, true),
            Msg::KeyUp(key_code) => self.keyboard.set_pressed(*key_code, false),
        }
    }
}

pub enum LuaMsg {
    Load(String),
}
pub enum Msg {
    AdvanceClock(f32),
    MouseDown(i32, i32),
    MouseUp,
    MouseMove(i32, i32),
    KeyDown(KeyCode),
    KeyUp(KeyCode),
    Zoom(f32),
}
