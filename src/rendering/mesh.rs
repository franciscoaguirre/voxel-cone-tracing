use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr;

use c_str_macro::c_str;
use cgmath::prelude::*;
use cgmath::{Vector2, Vector3};
use memoffset::offset_of;

use super::shader::Shader;

// NOTE: without repr(C) the compiler may reorder the fields or use different padding/alignment than C.
// Depending on how you pass the data to OpenGL, this may be bad. In this case it's not strictly
// necessary though because of the `offset!` macro used below in setup_mesh()
#[repr(C, packed)]
pub struct Vertex {
    // position
    pub position: Vector3<f32>,
    // normal
    pub normal: Vector3<f32>,
    // texCoords
    pub texture_coordinates: Vector2<f32>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::zero(),
            normal: Vector3::zero(),
            texture_coordinates: Vector2::zero(),
        }
    }
}

#[derive(Clone)]
pub struct Texture {
    pub id: u32,
    pub type_: String,
    pub path: String,
}

pub struct Mesh {
    /*  Mesh Data  */
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub vao: u32,

    /*  Render data  */
    vbo: u32,
    ebo: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Mesh {
        let mut mesh = Mesh {
            vertices,
            indices,
            textures,
            vao: 0,
            vbo: 0,
            ebo: 0,
        };

        // now that we have all the required data, set the vertex buffers and its attribute pointers.
        unsafe { mesh.setup_mesh() }
        mesh
    }

    /// render the mesh
    pub unsafe fn draw(&self, shader: &Shader) {
        // bind appropriate textures
        let mut num_diffuse = 0;
        let mut num_specular = 0;
        let mut num_normal = 0;
        let mut num_height = 0;
        if !self.textures.is_empty() {
            shader.set_bool(c_str!("hasTexture"), true);
        }
        for (i, texture) in self.textures.iter().enumerate() {
            gl::ActiveTexture(gl::TEXTURE0 + i as u32); // active proper texture unit before binding
                                                        // retrieve texture number (the N in diffuse_textureN)
            let name = &texture.type_;
            let number = match name.as_str() {
                "texture_diffuse" => {
                    num_diffuse += 1;
                    num_diffuse
                }
                "texture_specular" => {
                    num_specular += 1;
                    num_specular
                }
                "texture_normal" => {
                    num_normal += 1;
                    num_normal
                }
                "texture_height" => {
                    num_height += 1;
                    num_height
                }
                _ => panic!("unknown texture type"),
            };
            // now set the sampler to the correct texture unit
            // TODO: Use shader struct
            let sampler = CString::new(format!("{}{}", name, number)).unwrap();
            gl::Uniform1i(
                gl::GetUniformLocation(shader.id, sampler.as_ptr()),
                i as i32,
            );
            // and finally bind the texture
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
        }

        // draw mesh
        gl::BindVertexArray(self.vao);
        gl::DrawElements(
            gl::TRIANGLES,
            self.indices.len() as i32,
            gl::UNSIGNED_INT,
            ptr::null(),
        );
        gl::BindVertexArray(0);

        // always good practice to set everything back to defaults once configured.
        gl::ActiveTexture(gl::TEXTURE0);
    }

    unsafe fn setup_mesh(&mut self) {
        // create buffers/arrays
        gl::GenVertexArrays(1, &mut self.vao);
        gl::GenBuffers(1, &mut self.vbo);
        gl::GenBuffers(1, &mut self.ebo);

        gl::BindVertexArray(self.vao);
        // load data into vertex buffers
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        // A great thing about structs with repr(C) is that their memory layout is sequential for all its items.
        // The effect is that we can simply pass a pointer to the struct and it translates perfectly to a glm::vec3/2 array which
        // again translates to 3/2 floats which translates to a byte array.
        let size = (self.vertices.len() * size_of::<Vertex>()) as isize;
        let data = &self.vertices[0] as *const Vertex as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        let size = (self.indices.len() * size_of::<u32>()) as isize;
        let data = &self.indices[0] as *const u32 as *const c_void;
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        // set the vertex attribute pointers
        let size = size_of::<Vertex>() as i32;
        // vertex positions
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, position) as *const c_void,
        );
        // vertex normals
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, normal) as *const c_void,
        );
        // vertex texture coords
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, texture_coordinates) as *const c_void,
        );

        gl::BindVertexArray(0);
    }
}
