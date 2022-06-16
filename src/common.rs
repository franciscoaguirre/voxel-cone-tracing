use crate::{camera::Camera_Movement, Camera};
use glfw::{Action, Key};
use std::sync::mpsc::Receiver;

pub fn process_events(
    events: &Receiver<(f64, glfw::WindowEvent)>,
    first_mouse: &mut bool,
    last_x: &mut f32,
    last_y: &mut f32,
    camera: &mut Camera,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::CursorPos(x_position, y_position) => {
                let (x_position, y_position) = (x_position as f32, y_position as f32);
                if *first_mouse {
                    *last_x = x_position;
                    *last_y = y_position;
                    *first_mouse = false;
                }

                let x_offset = x_position - *last_x;
                let y_offset = *last_y - y_position; // reversed since y-coordinates go from bottom to top

                *last_x = x_position;
                *last_y = y_position;

                camera.ProcessMouseMovement(x_offset, y_offset, true);
            }
            glfw::WindowEvent::Scroll(_x_offset, y_offset) => {
                camera.ProcessMouseScroll(y_offset as f32);
            }
            _ => {}
        }
    }
}

pub fn process_input(window: &mut glfw::Window, delta_time: f32, camera: &mut Camera) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }

    if window.get_key(Key::W) == Action::Press {
        camera.ProcessKeyboard(Camera_Movement::Forward, delta_time);
    }
    if window.get_key(Key::S) == Action::Press {
        camera.ProcessKeyboard(Camera_Movement::Backward, delta_time);
    }
    if window.get_key(Key::A) == Action::Press {
        camera.ProcessKeyboard(Camera_Movement::Left, delta_time);
    }
    if window.get_key(Key::D) == Action::Press {
        camera.ProcessKeyboard(Camera_Movement::Right, delta_time);
    }
}
