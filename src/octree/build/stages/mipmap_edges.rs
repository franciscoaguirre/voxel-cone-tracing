use c_str_macro::c_str;

use crate::{
    config::CONFIG,
    helpers,
    octree::{build::BrickPoolValues, OctreeTextures},
    rendering::shader::Shader,
};

pub struct MipmapEdgesPass {
    shader: Shader,
}

impl MipmapEdgesPass {
    pub fn init() -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/mipmapEdges.comp.glsl"),
        }
    }

    pub unsafe fn run(
        &self,
        textures: &OctreeTextures,
        nodes_per_level: &[u32],
        level: u32,
        brick_pool_values: BrickPoolValues,
    ) {
        self.shader.use_program();

        self.shader.set_uint(c_str!("octreeLevel"), level);

        helpers::bind_image_texture(0, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);
        match brick_pool_values {
            BrickPoolValues::Colors => helpers::bind_3d_image_texture(
                1,
                textures.brick_pool_colors,
                gl::READ_WRITE,
                gl::RGBA8,
            ),
            BrickPoolValues::Normals => helpers::bind_3d_image_texture(
                1,
                textures.brick_pool_normals,
                gl::READ_WRITE,
                gl::RGBA8,
            ),
        }
        helpers::bind_image_texture(2, textures.level_start_indices.0, gl::READ_ONLY, gl::R32UI);

        let nodes_in_level = nodes_per_level[level as usize];
        let groups_count = (nodes_in_level as f32 / CONFIG.working_group_size as f32).ceil() as u32;

        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
