use c_str_macro::c_str;
use gl::types::*;

use crate::{config::CONFIG, rendering::shader::Shader, voxelization::helpers};

use super::common::{BRICK_POOL_COLORS_TEXTURE, OCTREE_NODE_POOL, OCTREE_NODE_POOL_BRICK_POINTERS};

pub struct MipmapCenterPass {
    shader: Shader,
    voxel_positions_texture: GLuint,
}

impl MipmapCenterPass {
    pub fn init(voxel_positions_texture: GLuint) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/mipmap_center.comp.glsl"),
            voxel_positions_texture,
        }
    }

    pub unsafe fn run(&self, level: u32) {
        self.shader.use_program();

        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);
        self.shader.set_uint(c_str!("maxOctreeLevel"), level);

        helpers::bind_image_texture(0, OCTREE_NODE_POOL.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_image_texture(
            1,
            OCTREE_NODE_POOL_BRICK_POINTERS.0,
            gl::READ_ONLY,
            gl::R32UI,
        );
        helpers::bind_3d_image_texture(2, BRICK_POOL_COLORS_TEXTURE, gl::READ_WRITE, gl::RGBA8);
        helpers::bind_image_texture(3, self.voxel_positions_texture, gl::READ_ONLY, gl::R32UI);

        let (debug_texture, debug_texture_buffer) =
            helpers::generate_texture_buffer(10, gl::R32F, 10_f32);
        helpers::bind_image_texture(4, debug_texture, gl::WRITE_ONLY, gl::R32F);

        // self.shader.dispatch(65_535); // TODO: Calculate number of groups
        self.shader.dispatch(1);
        self.shader.wait();

        let values = helpers::get_values_from_texture_buffer(debug_texture_buffer, 10, 11_f32);
        dbg!(&values);
    }
}
