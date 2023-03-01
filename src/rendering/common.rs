use std::{ffi::CStr, ptr, sync::mpsc::Receiver};

use egui_glfw_gl::glfw::{self, Action, Context, Glfw, Key, Window, WindowEvent};
use log::info;

use super::camera::{Camera, Camera_Movement};
use crate::{config::CONFIG, helpers::debug, voxelization::octree::visualize::ShowBricks};

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
        gl::DebugMessageCallback(Some(debug::gl_debug_output_callback), ptr::null());
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

            camera.ProcessMouseMovement(x_offset, y_offset, true);
        }
        glfw::WindowEvent::Scroll(_x_offset, y_offset) => {
            camera.ProcessMouseScroll(y_offset as f32);
        }
        _ => {}
    }
}

pub fn process_camera_input(window: &mut glfw::Window, delta_time: f32, camera: &mut Camera) {
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
    if window.get_key(Key::Space) == Action::Press {
        camera.ProcessKeyboard(Camera_Movement::Up, delta_time);
    }
    if window.get_key(Key::LeftShift) == Action::Press {
        camera.ProcessKeyboard(Camera_Movement::Down, delta_time);
    }
}

pub fn handle_update_octree_level(
    event: &glfw::WindowEvent,
    current_octree_level: &mut u32,
    show_empty_nodes: &mut bool,
    show_bricks: &mut ShowBricks,
) {
    match *event {
        glfw::WindowEvent::Key(Key::Left, _, Action::Press, _) => {
            if *current_octree_level != 0 {
                *current_octree_level -= 1
            }
            info!(
                "Current octree level: {} of {}",
                current_octree_level,
                CONFIG.octree_levels - 1
            );
        }
        glfw::WindowEvent::Key(Key::Right, _, Action::Press, _) => {
            *current_octree_level = (*current_octree_level + 1).min(CONFIG.octree_levels - 1);
            info!(
                "Current octree level: {} of {}",
                current_octree_level,
                CONFIG.octree_levels - 1
            );
        }
        glfw::WindowEvent::Key(Key::M, _, Action::Press, _) => {
            *show_empty_nodes = !*show_empty_nodes;
        }
        glfw::WindowEvent::Key(Key::B, _, Action::Press, _) => {
            *show_bricks = show_bricks.next();
            info!("Bricks: {}", *show_bricks);
        }
        _ => {}
    }
}

pub fn handle_showing_entities(
    event: &glfw::WindowEvent,
    show_model: &mut bool,
    show_voxel_fragment_list: &mut bool,
    show_octree: &mut bool,
) {
    match *event {
        glfw::WindowEvent::Key(Key::Num1, _, Action::Press, _) => {
            *show_model = !*show_model;
        }
        glfw::WindowEvent::Key(Key::Num2, _, Action::Press, _) => {
            *show_voxel_fragment_list = !*show_voxel_fragment_list;
        }
        glfw::WindowEvent::Key(Key::Num3, _, Action::Press, _) => {
            *show_octree = !*show_octree;
        }
        _ => {}
    }
}

pub unsafe fn show_device_information() {
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
    info!("GPU in use: {vendor}, {renderer}");

    let mut max_3d_texture_size = 0;
    unsafe { gl::GetIntegerv(gl::MAX_3D_TEXTURE_SIZE, &mut max_3d_texture_size) };
    info!("Maximum 3D texture size (by dimension): {max_3d_texture_size}");
}
