use serde::Deserialize;
use cgmath::{Point3, point3, Matrix4};

use crate::prelude::{
    Transform,
    RenderGizmo,
    Textures,
    LIGHT_MAP_BUFFERS,
    Framebuffer,
    Aabb,
    Object,
};

mod point;
use point::PointLight;

mod spot;
use spot::SpotLight;

#[derive(Debug, Deserialize, Clone)]
pub enum Light {
    Point(PointLight),
    Spot(SpotLight),
}

impl Light {
    pub unsafe fn new_point(color: Point3<f32>, intensity: f32) -> Self {
        Self::Point(PointLight::new(color, intensity))
    }

    pub unsafe fn new_spot(width: f32, height: f32, color: Point3<f32>, intensity: f32) -> Self {
        Self::Spot(SpotLight::new(width, height, color, intensity))
    }

    pub fn get_projection_matrix(&self) -> Matrix4<f32> {
        match self {
            Self::Point(point_light) => point_light.get_projection_matrix(),
            Self::Spot(spot_light) => spot_light.get_projection_matrix(),
        }
    }

    pub fn intensity(&self) -> f32 {
        match self {
            Self::Point(point_light) => point_light.intensity,
            Self::Spot(spot_light) => spot_light.intensity,
        }
    }

    pub fn transform(&self) -> &Transform {
        match self {
            Self::Point(point_light) => &point_light.transform,
            Self::Spot(spot_light) => &spot_light.transform,
        }
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        match self {
            Self::Point(point_light) => &mut point_light.transform,
            Self::Spot(spot_light) => &mut spot_light.transform,
        }
    }

    pub fn is_directional(&self) -> bool {
        match self {
            Self::Point(_) => false,
            Self::Spot(_) => true,
        }
    }

    pub unsafe fn take_photo(
        &self,
        objects: &mut [Object],
        scene_aabb: &Aabb,
        voxel_dimension: u32, // TODO: Find another way. This breaks separation of concerns
    ) -> Textures<LIGHT_MAP_BUFFERS> {
        gl::CullFace(gl::FRONT);
        let textures = match self {
            Self::Point(point_light) => point_light.take_photo(
                objects,
                scene_aabb,
                voxel_dimension
            ),
            Self::Spot(spot_light) => spot_light.take_photo(
                objects,
                scene_aabb,
                voxel_dimension
            ),
        };
        gl::CullFace(gl::BACK);
        textures
    }
}

impl Default for Light {
    fn default() -> Self {
        unsafe { Self::new_point(point3(1.0, 1.0, 1.0), 1_000_000.0) }
    }
}

impl RenderGizmo for Light {
    unsafe fn draw_gizmo(&self, projection: &Matrix4<f32>, view: &Matrix4<f32>) {
        match self {
            Self::Point(point_light) => point_light.draw_gizmo(projection, view),
            Self::Spot(spot_light) => spot_light.draw_gizmo(projection, view),
        }
    }
}
