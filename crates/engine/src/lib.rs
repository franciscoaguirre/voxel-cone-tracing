//! Simple rendering engine.
//! Can handle the usual Transforms, Lights, Cameras, etc.

pub mod camera;
pub mod common;
pub mod framebuffer;
pub mod gizmo;
pub mod light;
pub mod macros;
pub mod mesh;
pub mod model;
pub mod quad;
pub mod shader;
pub mod transform;
pub mod aabb;
pub mod helpers;
pub mod types;
pub mod scene;
pub mod material;
pub mod object;
pub mod asset_registry;
pub mod test_utils;

#[cfg(feature = "ui")]
pub mod ui;

pub mod prelude {
    pub use super::{
        transform::Transform,
        shader::{Shader, compile_shaders, compile_compute},
        helpers,
        types::*,
        camera::Camera,
        framebuffer::{
            Framebuffer,
            GeometryFramebuffer,
            GEOMETRY_BUFFERS,
            LightFramebuffer,
            LIGHT_MAP_BUFFERS,
        },
        light::SpotLight,
        model::Model,
        material::{Material, MaterialProperties},
        scene::{Scene, process_scene},
        object::Object,
        asset_registry::{AssetRegistry, AssetHandle},
        quad::Quad,
        aabb::Aabb,
        gizmo::RenderGizmo,
        common,
        test_utils,
    };
}
