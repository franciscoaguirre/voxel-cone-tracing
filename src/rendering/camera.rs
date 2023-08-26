use serde::Deserialize;

use super::transform::Transform;

// Default camera values
const SPEED: f32 = 1.0;
const SENSITIVITY: f32 = 0.1;
const ZOOM: f32 = 45.0;

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Camera {
    pub transform: Transform,
    #[serde(skip_deserializing, default = "default_yaw")]
    yaw: f32,
    #[serde(skip_deserializing)]
    pitch: f32,
    // Camera options
    #[serde(default = "default_speed")]
    pub movement_speed: f32,
    #[serde(default = "default_sensitivity")]
    pub mouse_sensitivity: f32,
    #[serde(default = "default_zoom")]
    pub zoom: f32,
}

const fn default_yaw() -> f32 {
    90.0
}

const fn default_speed() -> f32 {
    SPEED
}

const fn default_sensitivity() -> f32 {
    SENSITIVITY
}

const fn default_zoom() -> f32 {
    ZOOM
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
