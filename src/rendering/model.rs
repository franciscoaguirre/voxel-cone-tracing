#![allow(non_snake_case)]
#![allow(dead_code)]

use std::os::raw::c_void;
use std::path::Path;

use cgmath::{vec2, vec3};
use image::DynamicImage::*;

use super::mesh::{Mesh, Texture, Vertex};
use super::shader::Shader;
use crate::voxelization::aabb::Aabb;

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
        model.loadModel(path);
        model
    }

    pub fn Draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            unsafe {
                mesh.Draw(shader);
            }
        }
    }

    // loads a model from file and stores the resulting meshes in the meshes vector.
    fn loadModel(&mut self, path: &str) {
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
            let (p, _n, t) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..num_vertices {
                let pos_x = p[i * 3];
                let pos_y = p[i * 3 + 1];
                let pos_z = p[i * 3 + 2];

                let normal = if i * 3 + 2 < t.len() {
                    vec3(t[i * 3], t[i * 3 + 1], t[i * 3 + 2])
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
                    Position: vec3(pos_x, pos_y, pos_z),
                    Normal: normal,
                    TexCoords: tex_coords,
                })
            }

            // process material
            let mut textures = Vec::new();
            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];

                // 1. diffuse map
                if !material.diffuse_texture.is_empty() {
                    let texture =
                        self.loadMaterialTexture(&material.diffuse_texture, "texture_diffuse");
                    textures.push(texture);
                }
                // 2. specular map
                if !material.specular_texture.is_empty() {
                    let texture =
                        self.loadMaterialTexture(&material.specular_texture, "texture_specular");
                    textures.push(texture);
                }
                // 3. normal map
                if !material.normal_texture.is_empty() {
                    let texture =
                        self.loadMaterialTexture(&material.normal_texture, "texture_normal");
                    textures.push(texture);
                }
                // NOTE: no height maps
            }

            self.meshes.push(Mesh::new(vertices, indices, textures));
        }
        self.aabb = aabb;
    }

    fn loadMaterialTexture(&mut self, path: &str, typeName: &str) -> Texture {
        {
            let texture = self.textures_loaded.iter().find(|t| t.path == path);
            if let Some(texture) = texture {
                return texture.clone();
            }
        }

        let texture = Texture {
            id: unsafe { TextureFromFile(path, &self.directory) },
            type_: typeName.into(),
            path: path.into(),
        };
        self.textures_loaded.push(texture.clone());
        texture
    }
}

unsafe fn TextureFromFile(path: &str, directory: &str) -> u32 {
    let filename = format!("{}/{}", directory, path);

    let mut textureID = 0;
    gl::GenTextures(1, &mut textureID);

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

    gl::BindTexture(gl::TEXTURE_2D, textureID);
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

    textureID
}
