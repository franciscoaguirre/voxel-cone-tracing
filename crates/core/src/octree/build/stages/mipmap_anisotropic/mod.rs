mod mipmap_center;
mod mipmap_corners;
mod mipmap_edges;
mod mipmap_faces;

use mipmap_center::MipmapCenterPass;
use mipmap_corners::MipmapCornersPass;
use mipmap_edges::MipmapEdgesPass;
use mipmap_faces::MipmapFacesPass;

use crate::{
    constants::Direction,
    octree::{build::BrickPoolValues, NodeData, OctreeTextures},
};

pub struct MipmapAnisotropicPass {
    center: MipmapCenterPass,
    corners: MipmapCornersPass,
    edges: MipmapEdgesPass,
    faces: MipmapFacesPass,
}

impl MipmapAnisotropicPass {
    pub fn init() -> Self {
        Self {
            center: MipmapCenterPass::init(),
            corners: MipmapCornersPass::init(),
            edges: MipmapEdgesPass::init(),
            faces: MipmapFacesPass::init(),
        }
    }

    pub unsafe fn run(
        &self,
        textures: &OctreeTextures,
        node_data: &NodeData,
        level: u32,
        direction: Direction,
        brick_pool_values: BrickPoolValues,
    ) {
        self.center
            .run(textures, node_data, level, direction, brick_pool_values);
        self.corners
            .run(textures, node_data, level, direction, brick_pool_values);
        self.edges
            .run(textures, node_data, level, direction, brick_pool_values);
        self.faces
            .run(textures, node_data, level, direction, brick_pool_values);
    }
}
