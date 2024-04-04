//! Simple rendering engine.
//! Can handle the usual Transforms, Lights, Cameras, etc.

pub mod aabb;
pub mod asset_registry;
pub mod camera;
pub mod common;
pub mod cube;
pub mod decl_macros;
pub mod framebuffer;
pub mod gizmo;
pub mod helpers;
pub mod kernel;
pub mod kernels;
pub mod light;
pub mod material;
pub mod mesh;
pub mod model;
pub mod object;
pub mod quad;
pub mod render_loop;
pub mod scene;
pub mod shader;
pub mod test_utils;
pub mod texture_3d;
pub mod texture_buffer;
mod traits;
pub mod transform;
pub mod types;

pub use macros;

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
        kernel::{Kernel, Pausable},
        kernels,
        light::Light,
        macros::{aggregated_kernel, kernel_group, pausable},
        material::{Material, MaterialProperties},
        model::Model,
        object::Object,
        quad::Quad,
        render_loop::RenderLoop,
        scene::Scene,
        shader::{compile_compute, compile_shaders, Shader, ShaderPass},
        test_utils,
        texture_3d::Texture3Dv2,
        texture_buffer::BufferTextureV2,
        transform::Transform,
        types::*,
    };
}
