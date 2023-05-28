use c_str_macro::c_str;
use gl::types::GLuint;

use crate::{
    config::CONFIG,
    helpers,
    octree::{NodeData, OctreeTextures},
    rendering::shader::Shader,
};

pub struct BorderTransferPass {
    shader: Shader,
    light_view_map: GLuint,
}

impl BorderTransferPass {
    pub fn init(light_view_map: GLuint) -> Self {
        Self {
            shader: Shader::new_compute("assets/shaders/octree/lightTransfer.comp.glsl"),
            light_view_map,
        }
    }

    pub unsafe fn run(&self, textures: &OctreeTextures, octree_level: u32, node_data: &NodeData) {
        self.shader.use_program();

        self.shader.set_uint(c_str!("octreeLevel"), octree_level);
        self.shader
            .set_uint(c_str!("voxelDimension"), CONFIG.voxel_dimension);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, self.light_view_map);
        self.shader.set_int(c_str!("lightViewMap"), 0);

        helpers::bind_3d_image_texture(1, textures.brick_pool_photons, gl::READ_WRITE, gl::R32UI);
        helpers::bind_image_texture(2, textures.node_pool.0, gl::READ_ONLY, gl::R32UI);
        helpers::bind_image_texture(3, node_data.level_start_indices.0, gl::READ_ONLY, gl::R32UI);

        helpers::bind_3d_image_texture(0, textures.neighbors, gl::READ_ONLY, gl::R32UI);
        self.shader.set_uint(c_str!("axis"), 0);
        // self.shader.dispatch_xyz(vec3(
        //     (CONFIG.viewport_width as f32 / 32 as f32).ceil() as u32,
        //     (CONFIG.viewport_height as f32 / 32 as f32).ceil() as u32,
        //     1,
        // ));
        let nodes_in_level = node_data.nodes_per_level[octree_level as usize];
        let groups_count = (nodes_in_level as f32 / CONFIG.working_group_size as f32).ceil() as u32;
        self.shader.dispatch(groups_count);
        self.shader.wait();

        helpers::bind_3d_image_texture(0, textures.neighbors, gl::READ_ONLY, gl::R32UI);
        self.shader.set_uint(c_str!("axis"), 1);
        // self.shader.dispatch_xyz(vec3(
        //     (CONFIG.viewport_width as f32 / 32 as f32).ceil() as u32,
        //     (CONFIG.viewport_height as f32 / 32 as f32).ceil() as u32,
        //     1,
        // ));
        self.shader.dispatch(groups_count);
        self.shader.wait();

        helpers::bind_3d_image_texture(0, textures.neighbors, gl::READ_ONLY, gl::R32UI);
        self.shader.set_uint(c_str!("axis"), 2);
        // self.shader.dispatch_xyz(vec3(
        //     (CONFIG.viewport_width as f32 / 32 as f32).ceil() as u32,
        //     (CONFIG.viewport_height as f32 / 32 as f32).ceil() as u32,
        //     1,
        // ));
        self.shader.dispatch(groups_count);
        self.shader.wait();
    }
}
