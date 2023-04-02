use super::transform::Transform;

// Defines several possible options for camera movement. Used as abstraction to stay away from window-system specific input methods
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}
use self::CameraMovement::*;

// Default camera values
const SPEED: f32 = 1.0;
const SENSITIVITY: f32 = 0.1;
const ZOOM: f32 = 45.0;

pub struct Camera {
    pub transform: Transform,
    yaw: f32,
    pitch: f32,
    // Camera options
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            yaw: 90.0,
            pitch: 0.0,
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVITY,
            zoom: ZOOM,
        }
    }
}

impl Camera {
    /// Processes input received from any keyboard-like input system. Accepts input parameter in the form of camera defined ENUM (to abstract it from windowing systems)
    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.movement_speed * delta_time;
        if direction == Forward {
            self.transform.position += self.transform.get_forward() * velocity;
        }
        if direction == Backward {
            self.transform.position += -(self.transform.get_forward() * velocity);
        }
        if direction == Left {
            self.transform.position += -(self.transform.get_right() * velocity);
        }
        if direction == Right {
            self.transform.position += self.transform.get_right() * velocity;
        }
        if direction == Up {
            self.transform.position += self.transform.get_up() * velocity;
        }
        if direction == Down {
            self.transform.position += -(self.transform.get_up() * velocity);
        }
    }

    /// Processes input received from a mouse input system. Expects the offset value in both the x and y direction.
    pub fn process_mouse_movement(
        &mut self,
        mut xoffset: f32,
        mut yoffset: f32,
        constrain_pitch: bool,
    ) {
        xoffset *= self.mouse_sensitivity;
        yoffset *= self.mouse_sensitivity;

        self.yaw += xoffset;
        self.pitch += yoffset;

        // Make sure that when pitch is out of bounds, screen doesn't get flipped
        if constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        self.transform.set_rotation_x(self.pitch);
        self.transform.set_rotation_y(self.yaw);
    }

    // Processes input received from a mouse scroll-wheel event. Only requires input on the vertical wheel-axis
    pub fn process_mouse_scroll(&mut self, yoffset: f32) {
        if self.zoom >= 1.0 && self.zoom <= 45.0 {
            self.zoom -= yoffset;
        }
        if self.zoom <= 1.0 {
            self.zoom = 1.0;
        }
        if self.zoom >= 45.0 {
            self.zoom = 45.0;
        }
    }
}
