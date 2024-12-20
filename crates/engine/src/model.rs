use std::os::raw::c_void;
use std::path::Path;

use cgmath::{vec2, vec3};
use image::DynamicImage::*;

use super::mesh::{Mesh, Texture, Vertex};
use super::shader::Shader;
use crate::aabb::Aabb;

#[derive(Default)]
pub struct Model {
    /*  Model Data */
    pub meshes: Vec<Mesh>,
    pub textures_loaded: Vec<Texture>, // stores all the textures loaded so far, optimization to make sure textures aren't loaded more than once.
    pub aabb: Aabb,
    directory: String,
}

impl Model {
    /// constructor, expects a filepath to a 3D model.
    pub fn new(path: &str) -> Model {
        let mut model = Model::default();
        model.load_model(path);
        model
    }

    pub fn draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            unsafe {
                mesh.draw(shader);
            }
        }
    }

    // loads a model from file and stores the resulting meshes in the meshes vector.
    fn load_model(&mut self, path: &str) {
        let path = Path::new(path);

        // retrieve the directory path of the filepath
        self.directory = path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_str()
            .unwrap()
            .into();
        let obj = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS);

        let (models, materials) = obj.expect("Failed to load OBJ file");
        let materials = materials.expect("Failed to load MTL file");
        let mut aabb = Aabb::default();

        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            // data to fill
            let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
            let indices: Vec<u32> = mesh.indices.clone();
            let (p, n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..num_vertices {
                let pos_x = p[i * 3];
                let pos_y = p[i * 3 + 1];
                let pos_z = p[i * 3 + 2];

                let normal = if i * 3 + 2 < n.len() {
                    vec3(n[i * 3], n[i * 3 + 1], n[i * 3 + 2])
                } else {
                    vec3(0.0, 0.0, 0.0)
                };

                let tex_coords = if i * 2 + 1 < t.len() {
                    vec2(t[i * 2], t[i * 2 + 1])
                } else {
                    vec2(0.0, 0.0)
                };

                aabb.refresh_aabb(pos_x, pos_y, pos_z);

                vertices.push(Vertex {
                    position: vec3(pos_x, pos_y, pos_z),
                    normal,
                    texture_coordinates: tex_coords,
                })
            }

            // process material
            let mut textures = Vec::new();
            let mut diffuse: Option<[f32; 3]> = None;
            let mut specular: Option<[f32; 3]> = None;
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];

                if !material.diffuse.is_empty() {
                    diffuse = Some(material.diffuse);
                }

                if !material.specular.is_empty() {
                    specular = Some(material.specular);
                }

                // 1. diffuse map
                if !material.diffuse_texture.is_empty() {
                    let texture =
                        self.load_material_texture(&material.diffuse_texture, "texture_diffuse");
                    textures.push(texture);
                }
                // 2. specular map
                if !material.specular_texture.is_empty() {
                    let texture =
                        self.load_material_texture(&material.specular_texture, "texture_specular");
                    textures.push(texture);
                }
                // 3. normal map
                if !material.normal_texture.is_empty() {
                    let texture =
                        self.load_material_texture(&material.normal_texture, "texture_normal");
                    textures.push(texture);
                }
                // NOTE: no height maps
            }

            self.meshes
                .push(Mesh::new(vertices, indices, textures, diffuse, specular));
        }
        self.aabb = aabb;
    }

    fn load_material_texture(&mut self, path: &str, type_name: &str) -> Texture {
        {
            let texture = self.textures_loaded.iter().find(|t| t.path == path);
            if let Some(texture) = texture {
                return texture.clone();
            }
        }

        let texture = Texture {
            id: unsafe { texture_from_file(path) },
            type_: type_name.into(),
            path: path.into(),
        };
        self.textures_loaded.push(texture.clone());
        texture
    }
}

unsafe fn texture_from_file(filename: &str) -> u32 {
    let mut texture_id = 0;
    gl::GenTextures(1, &mut texture_id);

    use std::env;

    let img = image::open(&Path::new(&filename)).expect("Texture failed to load");
    let img = img.flipv();
    let format = match img {
        ImageLuma8(_) => gl::RED,
        ImageLumaA8(_) => gl::RG,
        ImageRgb8(_) => gl::RGB,
        ImageRgba8(_) => gl::RGBA,
        _ => panic!("Unsupported image format found"),
    };

    let width = img.width();
    let height = img.height();
    let data = img.into_bytes();

    gl::BindTexture(gl::TEXTURE_2D, texture_id);
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        format as i32,
        width as i32,
        height as i32,
        0,
        format,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void,
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR_MIPMAP_LINEAR as i32,
    );
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    texture_id
}
