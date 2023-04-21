use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Vector3};
use std::f32::consts::PI;

use super::keyboard::{KeyCode, Keyboard};

pub struct Camera {
    projection: Perspective3<f32>,
    left_right_radians: f32,
    up_down_radians: f32,
    orbit_radius: f32,
    pos_x: f32,
    pos_z: f32,
}

enum Mode {
    Orbit,
    FPS,
}
const MODE: Mode = Mode::Orbit;

impl Camera {
    pub fn new() -> Camera {
        let fovy = 16.0 / 9.0;

        match MODE {
            Mode::FPS => Camera {
                projection: Perspective3::new(fovy, 1.0, 0.1, 400.0),
                left_right_radians: 45.0f32.to_radians(),
                up_down_radians: 80.0f32.to_radians(),
                orbit_radius: 0.02,
                pos_x: 0.,
                pos_z: 0.,
            },
            Mode::Orbit => Camera {
                projection: Perspective3::new(fovy, 1.0, 0.1, 400.0),
                left_right_radians: 45.0f32.to_radians(),
                up_down_radians: 80.0f32.to_radians(),
                orbit_radius: 15.,
                pos_x: 0.,
                pos_z: 0.,
            },
        }
    }

    pub fn update_aspect(&mut self, aspect: f32) {
        self.projection = Perspective3::new(aspect, 1.0, 0.1, 400.0);
    }

    pub fn view(&self) -> [f32; 16] {
        let eye = self.get_eye_pos();

        let target = Point3::new(self.pos_x, 0.0, self.pos_z);

        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

        let view = view.to_homogeneous();

        let mut view_array = [0.; 16];
        view_array.copy_from_slice(view.as_slice());

        view_array
    }

    pub fn view_mat(&self) -> Matrix4<f32> {
        let eye = self.get_eye_pos();

        let target = Point3::new(self.pos_x, 0.0, self.pos_z);

        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

        view.to_homogeneous()
    }

    pub fn projection_mat(&self) -> &Matrix4<f32> {
        self.projection.as_matrix()
    }
    pub fn view_flipped_y_mat(&self) -> Matrix4<f32> {
        let mut eye = self.get_eye_pos();
        eye.y = -1.0 * eye.y;

        let target = Point3::new(self.pos_x, 0.0, self.pos_z);

        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

        view.to_homogeneous()
    }
    pub fn view_flipped_y(&self) -> [f32; 16] {
        let mut eye = self.get_eye_pos();
        eye.y = -1.0 * eye.y;

        let target = Point3::new(self.pos_x, 0.0, self.pos_z);

        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

        let view = view.to_homogeneous();

        let mut view_array = [0.; 16];
        view_array.copy_from_slice(view.as_slice());

        view_array
    }

    pub fn get_eye_pos(&self) -> Point3<f32> {
        let yaw = self.left_right_radians;
        let pitch = self.up_down_radians;

        let eye_x = self.orbit_radius * yaw.sin() * pitch.cos();
        let eye_y = self.orbit_radius * pitch.sin();
        let eye_z = self.orbit_radius * yaw.cos() * pitch.cos();

        Point3::new(eye_x + self.pos_x, eye_y, eye_z + self.pos_z)
    }
    pub fn projection(&self) -> [f32; 16] {
        let mut perspective_array = [0.; 16];
        perspective_array.copy_from_slice(self.projection.as_matrix().as_slice());

        perspective_array
    }

    pub fn orbit_left_right(&mut self, delta: f32) {
        self.left_right_radians += delta;
    }

    pub fn orbit_up_down(&mut self, delta: f32) {
        self.up_down_radians += delta;

        // Make sure:
        // 0.1 <= radians <= PI / 2.1
        // in order to restrict the camera's up/down orbit motion

        match MODE {
            Mode::FPS => {
                if self.up_down_radians > PI / 2.1 {
                    self.up_down_radians = PI / 2.1;
                }

                if self.up_down_radians < -PI / 2.1 {
                    self.up_down_radians = -PI / 2.1;
                }
            }
            Mode::Orbit => {
                if self.up_down_radians > (PI / 2.1) {
                    self.up_down_radians = PI / 2.1;
                }

                if self.up_down_radians < 0.1 {
                    self.up_down_radians = 0.1;
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32, keyboard: &Keyboard) {
        if let Mode::FPS = MODE {
            let x = if keyboard.get_pressed(KeyCode::W) {
                dt * 0.005
            } else if keyboard.get_pressed(KeyCode::S) {
                -dt * 0.005
            } else {
                0.
            };
            let z = if keyboard.get_pressed(KeyCode::A) {
                dt * 0.005
            } else if keyboard.get_pressed(KeyCode::D) {
                -dt * 0.005
            } else {
                0.
            };

            let view = self.view_mat();

            let dir = view.transform_vector(&Vector3::new(x, 0.0, z));

            self.pos_x -= dir.z;
            self.pos_z -= dir.x;
        }
    }

    pub fn zoom(&mut self, zoom: f32) {
        if let Mode::Orbit = MODE {
            self.orbit_radius += zoom;

            if self.orbit_radius > 300. {
                self.orbit_radius = 300.;
            } else if self.orbit_radius < 5. {
                self.orbit_radius = 5.;
            }
        }
    }
}
