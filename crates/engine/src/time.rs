use egui_glfw_gl::glfw::Glfw;

pub struct TimeManager {
    delta_time: f64,
    last_frame: f64,
}

impl TimeManager {
    pub fn new() -> Self {
        Self {
            delta_time: 0.0,
            last_frame: 0.0,
        }
    }

    pub fn update(&mut self, glfw: &Glfw) -> f64 {
        let current_frame = glfw.get_time();
        self.delta_time = current_frame - self.last_frame;
        self.last_frame = current_frame;
        current_frame
    }

    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }
}
