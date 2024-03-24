//! Simple rendering engine.
//! Can handle the usual Transforms, Lights, Cameras, etc.

pub mod aabb;
pub mod asset_registry;
pub mod camera;
pub mod common;
pub mod cube;
pub mod framebuffer;
pub mod gizmo;
pub mod helpers;
pub mod light;
pub mod macros;
pub mod material;
pub mod mesh;
pub mod model;
pub mod object;
pub mod quad;
pub mod scene;
pub mod shader;
pub mod test_utils;
pub mod texture_3d;
pub mod texture_buffer;
mod traits;
pub mod transform;
pub mod types;

#[cfg(feature = "ui")]
pub mod ui;

pub mod prelude {
    pub use super::{
        aabb::Aabb,
        asset_registry::{AssetHandle, AssetRegistry},
        camera::Camera,
        common,
        cube::Cube,
        framebuffer::{
            Framebuffer, GeometryFramebuffer, LightFramebuffer, GEOMETRY_BUFFERS, LIGHT_MAP_BUFFERS,
        },
        gizmo::RenderGizmo,
        helpers,
        light::Light,
        material::{Material, MaterialProperties},
        model::Model,
        object::Object,
        quad::Quad,
        scene::{process_scene, Scene},
        shader::{compile_compute, compile_shaders, Shader, ShaderPass},
        test_utils,
        texture_3d::Texture3Dv2,
        texture_buffer::BufferTextureV2,
        transform::Transform,
        types::*,
    };
}
