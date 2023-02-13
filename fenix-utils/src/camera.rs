//! Camera module.

use glam::{Mat4, Vec3};
use log::debug;

// Default values
const YAW: f32         = 0.0;
const PITCH: f32       = 0.0;
const SPEED: f32       = 2.5;
const SENSITIVITY: f32 = 0.1;
const FOV: f32         = 45.0;
const A_RATIO: f32     = 16.0 / 9.0;
const Z_NEAR: f32      = 0.1;
const Z_FAR: f32       = 10.0;

pub enum CameraMotion {
    Forward,
    Backward,
    Right,
    Left,
    Up,
    Down,
}

#[rustfmt::skip]
pub struct FlyCamera {
    position:     Vec3,
    world_up:     Vec3,
    yaw:          f32,
    pitch:        f32,

    // fov:          f32,
    // aspect_ratio: f32,
    // z_near:       f32,
    // z_far:        f32,

    speed:        f32,
    sensitivity:  f32,

    back:         Vec3,
    right:        Vec3,
    up:           Vec3,

    view_matrix:  Mat4,
}

impl FlyCamera {
    /// Creates a camera with sensible default values.
    pub fn new() -> Self {
        let mut camera = Self {
            position: Vec3::ZERO,
            world_up: Vec3::Y,
            yaw: YAW,
            pitch: PITCH,
            speed: SPEED,
            sensitivity: SENSITIVITY,
            back: Vec3::ZERO,
            right: Vec3::ZERO,
            up: Vec3::ZERO,
            view_matrix: Mat4::ZERO,
        };
        camera.update_vectors();
        camera.update_view_matrix();
        camera
    }

    /// Creates a camera with custom parameters.
    pub fn from(position: Vec3, world_up: Vec3, yaw: f32, pitch: f32) -> Self {
        let mut camera = Self {
            position,
            world_up,
            yaw,
            pitch,
            speed: SPEED,
            sensitivity: SENSITIVITY,
            back: Vec3::ZERO,
            right: Vec3::ZERO,
            up: Vec3::ZERO,
            view_matrix: Mat4::ZERO,
        };
        camera.update_vectors();
        camera.update_view_matrix();
        camera
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        self.view_matrix
    }

    fn update_vectors(&mut self) {
        let yaw = self.yaw.to_radians();
        let pitch = self.pitch.to_radians();

        let back = Vec3 {
            x: yaw.cos() * pitch.cos(),
            y: pitch.sin(),
            z: yaw.sin() * pitch.cos(),
        };
        let right = back.cross(self.world_up).normalize();
        let up = right.cross(back).normalize();

        self.back = back;
        self.right = right;
        self.up = up;
    }

    fn update_view_matrix(&mut self) {
        let rotation = Mat4::from_cols_array(&[
            self.right.x, self.up.x, self.back.x, 0.0, // col 1
            self.right.y, self.up.y, self.back.y, 0.0, // col 2
            self.right.z, self.up.z, self.back.z, 0.0, // col 3
            0.0,          0.0,       0.0,         1.0, // col 4
        ]);
        let translation = Mat4::from_cols_array(&[
             1.0,              0.0,              0.0,             0.0,
             0.0,              1.0,              0.0,             0.0,
             0.0,              0.0,              1.0,             0.0,
            -self.position.x, -self.position.y, -self.position.z, 1.0,
        ]);

        self.view_matrix = rotation * translation;
    }

    pub fn process_move_action(&mut self, motion_dir: CameraMotion, dt: f32) {
        let distance = self.speed * dt;
        match motion_dir {
            CameraMotion::Backward => self.position += distance * self.back,
            CameraMotion::Forward  => self.position -= distance * self.back,
            CameraMotion::Right    => self.position += distance * self.right,
            CameraMotion::Left     => self.position -= distance * self.right,
            CameraMotion::Up       => self.position += distance * self.up,
            CameraMotion::Down     => self.position -= distance * self.up,
        }
        self.update_view_matrix();
    }

    pub fn process_mouse_movement(&mut self, x_offset: f32, y_offset: f32, constrain_pitch: bool) {
        self.yaw   += (x_offset * self.sensitivity) % 360.0; // Avoid losing precission if yaw gets too big
        self.pitch += y_offset * self.sensitivity;

        if constrain_pitch {
            self.pitch = self.pitch.clamp(-89.0, 89.0);
        }

        self.update_vectors();
        self.update_view_matrix();
    }

    pub fn process_mouse_scroll(scroll_offset: f32) {
        debug!("{}", scroll_offset);
    }
}
