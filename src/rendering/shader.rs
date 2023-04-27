use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::{env, ptr, str};

use cgmath::{vec3, Matrix, Matrix4, Vector3};
use gl::types::*;
use log::trace;

#[derive(Default, Debug)]
pub struct Shader {
    pub id: u32,
    is_compute: bool,
}

enum ShaderStage {
    Vertex,
    Fragment,
    Geometry,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let mut shader = Shader::default();

        let vertex_code = Shader::process_shader_file(vertex_path);
        let fragment_code = Shader::process_shader_file(fragment_path);

        let short_vertex_path = &vertex_path[15..];
        trace!("Compiling shader in path {short_vertex_path}");
        unsafe {
            shader.id = Shader::compile_shaders(&vertex_code, &fragment_code, None);
        }

        shader
    }

    pub fn with_geometry_shader(
        vertex_path: &str,
        fragment_path: &str,
        geometry_path: &str,
    ) -> Self {
        let mut shader = Shader::default();

        let vertex_code = Shader::process_shader_file(vertex_path);
        let fragment_code = Shader::process_shader_file(fragment_path);
        let geometry_code = Shader::process_shader_file(geometry_path);

        let short_vertex_path = &vertex_path[15..];
        trace!("Compiling shader in path {short_vertex_path}");
        unsafe {
            shader.id = Shader::compile_shaders(&vertex_code, &fragment_code, Some(&geometry_code));
        }

        shader
    }

    pub fn new_compute(shader_path: &str) -> Self {
        let mut shader = Shader {
            id: 0,
            is_compute: true,
        };

        let shader_code = Shader::process_shader_file(shader_path);

        let short_shader_path = &shader_path[15..];
        trace!("Compiling shader in path {short_shader_path}");
        unsafe {
            shader.id = Shader::compile_compute(&shader_code);
        }

        shader
    }

    pub fn new_single(shader_path: &str) -> Self {
        let mut shader = Shader::default();

        let shader_code = Shader::process_shader_file(shader_path);
        let (vertex_code, fragment_code, geometry_code) =
            Shader::split_shader_file(shader_code.to_str().unwrap().to_string());

        let short_shader_path = &shader_path[15..];
        trace!("Compiling shader in path {short_shader_path}");
        unsafe {
            shader.id =
                Shader::compile_shaders(&vertex_code, &fragment_code, geometry_code.as_ref());
        }

        shader
    }

    /// Activate the shader
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id)
    }

    pub unsafe fn dispatch_xyz(&self, number_of_groups: Vector3<u32>) {
        if !self.is_compute {
            panic!("Can't dispatch a non-compute shader");
        }
        gl::DispatchCompute(number_of_groups.x, number_of_groups.y, number_of_groups.z);
    }

    pub unsafe fn dispatch(&self, number_of_groups: u32) {
        self.dispatch_xyz(vec3(number_of_groups, 1, 1));
    }

    pub unsafe fn wait(&self) {
        gl::MemoryBarrier(gl::ALL_BARRIER_BITS);
    }

    /// Utility uniform functions
    /// ------------------------------------------------------------------------
    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value as i32);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_uint(&self, name: &CStr, value: u32) {
        gl::Uniform1ui(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_vec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        gl::Uniform3f(gl::GetUniformLocation(self.id, name.as_ptr()), x, y, z);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_mat4(&self, name: &CStr, mat: &Matrix4<f32>) {
        gl::UniformMatrix4fv(
            gl::GetUniformLocation(self.id, name.as_ptr()),
            1,
            gl::FALSE,
            mat.as_ptr(),
        );
    }

    fn process_shader_file(file_path: &str) -> CString {
        let mut shader_file =
            File::open(file_path).unwrap_or_else(|_| panic!("Failed to open {}", file_path));
        let mut shader_code = String::new();
        shader_file
            .read_to_string(&mut shader_code)
            .unwrap_or_else(|_| panic!("Failed to read shader: {}", file_path));
        let file_directory = std::path::Path::new(file_path)
            .parent()
            .unwrap_or_else(|| panic!("Couldn't get parent of {}", file_path));
        shader_code = Shader::process_include_directive(shader_code, file_directory);
        CString::new(shader_code.as_bytes()).unwrap()
    }

    unsafe fn compile_shaders(
        vertex_code: &CString,
        fragment_code: &CString,
        geometry_code: Option<&CString>,
    ) -> u32 {
        let vertex = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex, 1, &vertex_code.as_ptr(), ptr::null());
        gl::CompileShader(vertex);
        Shader::check_compile_errors(vertex, "VERTEX");
        // fragment Shader
        let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment, 1, &fragment_code.as_ptr(), ptr::null());
        gl::CompileShader(fragment);
        Shader::check_compile_errors(fragment, "FRAGMENT");

        let mut geometry = 0;

        if let Some(geometry_code) = geometry_code {
            // geometry shader
            geometry = gl::CreateShader(gl::GEOMETRY_SHADER);
            gl::ShaderSource(geometry, 1, &geometry_code.as_ptr(), ptr::null());
            gl::CompileShader(geometry);
            Shader::check_compile_errors(geometry, "GEOMETRY");
        }

        // shader Program
        let id = gl::CreateProgram();
        gl::AttachShader(id, vertex);
        gl::AttachShader(id, fragment);

        if geometry != 0 {
            gl::AttachShader(id, geometry);
        }

        gl::LinkProgram(id);
        Shader::check_compile_errors(id, "PROGRAM");
        // delete the shaders as they're linked into our program now and no longer necessary
        gl::DeleteShader(vertex);
        gl::DeleteShader(fragment);

        if geometry != 0 {
            gl::DeleteShader(geometry);
        }

        id
    }

    unsafe fn compile_compute(shader_code: &CString) -> u32 {
        let shader_id = gl::CreateShader(gl::COMPUTE_SHADER);
        gl::ShaderSource(shader_id, 1, &shader_code.as_ptr(), ptr::null());
        gl::CompileShader(shader_id);
        Shader::check_compile_errors(shader_id, "COMPUTE");

        let program_id = gl::CreateProgram();
        gl::AttachShader(program_id, shader_id);
        gl::LinkProgram(program_id);
        Shader::check_compile_errors(program_id, "PROGRAM");

        gl::DeleteShader(shader_id);

        program_id
    }

    fn process_include_directive(shader_code: String, file_directory: &std::path::Path) -> String {
        let directive = "#include ";
        let previous_current_directory = env::current_dir().unwrap();
        let processed_shader_code = shader_code
            .lines()
            .map(|line| {
                if line.contains(directive) {
                    // + 1 and - 1 are to remove the quotes from the filename
                    let mut filename_to_include = &line[directive.len() + 1..line.len() - 1];
                    if filename_to_include.starts_with('.') {
                        env::set_current_dir(file_directory).unwrap();
                        // To get rid of the './'
                        filename_to_include = &line[directive.len() + 3..line.len() - 1];
                    }
                    let mut file_to_include = File::open(filename_to_include)
                        .unwrap_or_else(|_| panic!("Failed to open {}", filename_to_include));
                    let mut file_contents = String::new();
                    file_to_include
                        .read_to_string(&mut file_contents)
                        .unwrap_or_else(|_| panic!("Failed to read {}", filename_to_include));
                    env::set_current_dir(&previous_current_directory).unwrap();
                    return file_contents;
                }
                line.to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");

        processed_shader_code
    }

    /// Reads a unified shader file and returns the individual shader stages
    fn split_shader_file(shader_code: String) -> (CString, CString, Option<CString>) {
        let directive = "#shader ";

        let (vertex_code, fragment_code, geometry_code, _) = shader_code.lines().fold(
            (
                String::new(),
                String::new(),
                String::new(),
                ShaderStage::Vertex,
            ),
            |(mut vertex, mut fragment, mut geometry, mut shader_stage), line| {
                if line.contains(directive) {
                    shader_stage = match &line[directive.len()..line.len()] {
                        "vertex" => ShaderStage::Vertex,
                        "fragment" => ShaderStage::Fragment,
                        "geometry" => ShaderStage::Geometry,
                        _ => panic!("Shader directive: Unsupported shader stage"),
                    }
                } else {
                    // TODO: This is really inefficient
                    let line_to_push = format!("{line}\n");
                    match shader_stage {
                        ShaderStage::Vertex => vertex.push_str(&line_to_push),
                        ShaderStage::Fragment => fragment.push_str(&line_to_push),
                        ShaderStage::Geometry => geometry.push_str(&line_to_push),
                    }
                }

                (vertex, fragment, geometry, shader_stage)
            },
        );

        (
            CString::new(vertex_code.as_bytes()).unwrap(),
            CString::new(fragment_code.as_bytes()).unwrap(),
            if geometry_code.is_empty() {
                None
            } else {
                Some(CString::new(geometry_code.as_bytes()).unwrap())
            },
        )
    }

    /// utility function for checking shader compilation/linking errors.
    /// ------------------------------------------------------------------------
    unsafe fn check_compile_errors(shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = vec![0u8; 1024];
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                    type_,
                    str::from_utf8(&info_log).unwrap()
                );
            }
        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                    type_,
                    str::from_utf8(&info_log).unwrap()
                );
            }
        }
    }
}
