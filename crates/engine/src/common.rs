use std::{ffi::CStr, ptr, sync::{mpsc::Receiver}, cell::RefCell};

use egui_glfw_gl::glfw::{self, Action, Context, Glfw, Key, Window, WindowEvent};
use log;

use super::{
    camera::Camera,
    transform::{Direction, Transform},
};
use crate::{helpers, toggle_boolean};

#[cfg(feature = "ui")]
use crate::ui::Ui;

pub static mut WINDOW: RefCell<Option<glfw::Window>> = RefCell::new(None);

/// Panics if glfw hasn't been initialized yet
pub unsafe fn get_framebuffer_size() -> (i32, i32) {
    WINDOW.borrow().as_ref().unwrap().get_framebuffer_size()
}

unsafe fn set_window(window: Window) {
    *WINDOW.borrow_mut() = Some(window);
}

pub unsafe fn setup_glfw(viewport_width: i32, viewport_height: i32, debug: bool, headless: bool) -> (Glfw, Receiver<(f64, WindowEvent)>) {
    // GLFW: Setup
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(debug));
    glfw.window_hint(glfw::WindowHint::Visible(!headless));

    // GLFW: Window creation
    let (mut window, events) = glfw
        .create_window(
            viewport_width as u32,
            viewport_height as u32,
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

    #[cfg(feature = "ui")]
    Ui::setup(&mut window);

    set_window(window);

    (glfw, events)
}

pub fn swap_buffers() {
    unsafe { WINDOW.borrow_mut().as_mut().unwrap().swap_buffers(); }
}

pub fn process_events(
    event: &glfw::WindowEvent,
    first_mouse: &mut bool,
    last_x: &mut f32,
    last_y: &mut f32,
    camera: &mut Camera,
    // TODO: Bring back
    // debug_cone: &mut DebugCone,
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

            // TODO: Bring back
            // To be able to move the debug cone with the camera's forward
            // debug_cone.transform.set_rotation_y(camera.transform.rotation_y());
        }
        glfw::WindowEvent::Scroll(_x_offset, y_offset) => {
            camera.process_mouse_scroll(y_offset as f32);
        }
        _ => {}
    }
}

pub unsafe fn process_movement_input(
    delta_time: f32,
    transform: &mut Transform,
) {
    let binding = WINDOW.borrow();
    let window = binding.as_ref().unwrap();
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

pub fn should_close_window() -> bool {
    unsafe {
        let binding = WINDOW.borrow();
        let window = binding.as_ref().unwrap();
        window.should_close()
    }
}

toggle_boolean!(C, handle_light_movement);
toggle_boolean!(Num1, handle_show_model);
toggle_boolean!(Num2, handle_show_voxel_fragment_list);

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
    log::debug!("Maximum 3D texture size (by dimension): {max_3d_texture_size}");
}
