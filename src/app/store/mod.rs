mod mouse;

use nalgebra::Point3;
use nalgebra::Vector3;

use self::mouse::*;

mod camera;
use self::camera::*;

pub mod water;
use self::water::*;

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
    pub width: u32,
    pub height: u32,
    camera: Camera,
    mouse: Mouse,
    show_scenery: bool,
    pub entities: Vec<Box<Entity>>,
}

#[derive(Debug, Clone)]
pub enum Entity {
    EntMesh(std::rc::Rc<std::cell::RefCell<Mesh>>),
    EntWater(Water),
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub name: String,
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub update: Option<String>,
}

impl State {
    fn new() -> State {
        State {
            /// Time elapsed since the application started, in milliseconds
            clock: 0.,
            camera: Camera::new(),
            mouse: Mouse::default(),
            width: 1,
            height: 1,
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
            _ => (),
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
    Zoom(f32),
    SetReflectivity(f32),
    SetFresnel(f32),
    SetWaveSpeed(f32),
    UseReflection(bool),
    UseRefraction(bool),
    ShowScenery(bool),
}
