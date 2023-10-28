use std::path::PathBuf;
use std::env;

use engine::prelude::*;

#[test]
fn cone_trace_coordinates_make_sense() {
    let (_glfw, _window) = test_utils::init_opengl_context();

    // To go from the crate root to the workspace root
    let mut path = PathBuf::from(env::current_dir().unwrap());
    path.pop();
    path.pop();
    env::set_current_dir(path).unwrap();

    let shader = compile_compute!("assets/shaders/debug/simpleConeTrace.comp.glsl", debug = true);
    println!("Successfully compiled shader in test!");
}
