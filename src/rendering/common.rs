use std::{ffi::CStr, ptr, sync::mpsc::Receiver};

use egui_glfw_gl::glfw::{self, Action, Context, Glfw, Key, Window, WindowEvent};
use log;

use super::{
    camera::Camera,
    transform::{Direction, Transform},
};
use crate::{config::CONFIG, handle_increments, helpers, toggle_boolean};

pub unsafe fn setup_glfw(debug: bool) -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
    // GLFW: Setup
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(debug));

    // GLFW: Window creation
    let (mut window, events) = glfw
        .create_window(
            CONFIG.viewport_width as u32,
            CONFIG.viewport_height as u32,
            "Voxel Cone Tracing",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_char_polling(true);
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    // GL: Load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Enable OpenGL Debug Context if allowed
    let mut flags = 0;
    gl::GetIntegerv(gl::CONTEXT_FLAGS, &mut flags);
    if flags as u32 & gl::CONTEXT_FLAG_DEBUG_BIT != 0 {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageCallback(Some(helpers::gl_debug_output_callback), ptr::null());
        gl::DebugMessageControl(
            gl::DONT_CARE,
            gl::DONT_CARE,
            gl::DONT_CARE,
            0,
            ptr::null(),
            gl::TRUE,
        );
    } else {
        println!("Debug Context not active");
    }

    (glfw, window, events)
}

pub fn process_events(
    event: &glfw::WindowEvent,
    first_mouse: &mut bool,
    last_x: &mut f32,
    last_y: &mut f32,
    camera: &mut Camera,
) {
    match *event {
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

            camera.process_mouse_movement(x_offset, y_offset, true);
        }
        glfw::WindowEvent::Scroll(_x_offset, y_offset) => {
            camera.process_mouse_scroll(y_offset as f32);
        }
        _ => {}
    }
}

pub fn process_movement_input(
    window: &mut glfw::Window,
    delta_time: f32,
    transform: &mut Transform,
) {
    if window.get_key(Key::W) == Action::Press {
        transform.process_keyboard(Direction::Forward, delta_time);
    }
    if window.get_key(Key::S) == Action::Press {
        transform.process_keyboard(Direction::Backward, delta_time);
    }
    if window.get_key(Key::A) == Action::Press {
        transform.process_keyboard(Direction::Left, delta_time);
    }
    if window.get_key(Key::D) == Action::Press {
        transform.process_keyboard(Direction::Right, delta_time);
    }
    if window.get_key(Key::Space) == Action::Press {
        transform.process_keyboard(Direction::Up, delta_time);
    }
    if window.get_key(Key::LeftShift) == Action::Press {
        transform.process_keyboard(Direction::Down, delta_time);
    }
}

toggle_boolean!(C, handle_light_movement);
toggle_boolean!(Num1, handle_show_model);
toggle_boolean!(Num2, handle_show_voxel_fragment_list);
handle_increments!(
    "Cone angle",
    Up,
    Down,
    handle_cone_angle,
    f32,
    0.01,
    0.0,
    6.0
);

pub unsafe fn log_device_information() {
    let vendor = unsafe {
        CStr::from_ptr(gl::GetString(gl::VENDOR) as *const i8)
            .to_str()
            .unwrap()
    };
    let renderer = unsafe {
        CStr::from_ptr(gl::GetString(gl::RENDERER) as *const i8)
            .to_str()
            .unwrap()
    };
    log::info!("GPU in use: {vendor}, {renderer}");

    let mut max_3d_texture_size = 0;
    unsafe { gl::GetIntegerv(gl::MAX_3D_TEXTURE_SIZE, &mut max_3d_texture_size) };
    log::info!("Maximum 3D texture size (by dimension): {max_3d_texture_size}");
}
