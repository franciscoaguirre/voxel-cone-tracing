use std::{ffi::c_void, mem::size_of};

use gl::types::GLuint;
use log::debug;

use crate::{config::CONFIG, constants, helpers, rendering::shader::Shader};

mod build;
mod lighting;
mod visualize;

pub use visualize::BricksToShow;

type Texture = GLuint;
type Texture3D = GLuint;
type TextureBuffer = GLuint;
type BufferTexture = (Texture, TextureBuffer);

pub struct Octree {
    nodes_per_level: Vec<u32>,
    pub textures: OctreeTextures,
    voxel_data: VoxelData,
    renderer: Renderer,
}

pub struct OctreeTextures {
    pub node_pool: BufferTexture,
    brick_pointers: BufferTexture,
    pub node_positions: BufferTexture,
    neighbors: [BufferTexture; 6],
    level_start_indices: BufferTexture,
    brick_pool_colors: Texture3D,
    brick_pool_normals: Texture3D,
    brick_pool_photons: Texture3D,
}

pub struct VoxelData {
    voxel_positions: Texture,
    number_of_voxel_fragments: u32,
    voxel_colors: Texture,
    voxel_normals: Texture,
}

struct Renderer {
    vao: GLuint,
    node_count: u32,
    shader: Shader,
    bricks_shader: Shader,
    bricks_to_show: BricksToShow,
    node_positions_shader: Shader,
    node_neighbors_shader: Shader,
    node_bricks_shader: Shader,
}

impl Octree {
    /// Creates a Sparse Voxel Octree (SVO)
    pub unsafe fn new(
        voxel_positions_texture: GLuint,
        number_of_voxel_fragments: u32,
        voxel_colors_texture: GLuint,
        voxel_normals_texture: GLuint,
    ) -> Self {
        let max_node_pool_size = Self::get_max_node_pool_size();
        let max_node_pool_size_in_bytes = size_of::<GLuint>() * max_node_pool_size as usize;
        let textures = Self::initialize_textures(max_node_pool_size_in_bytes);
        let nodes_per_level = Vec::new();
        let voxel_data = VoxelData {
            voxel_positions: voxel_positions_texture,
            voxel_colors: voxel_colors_texture,
            voxel_normals: voxel_normals_texture,
            number_of_voxel_fragments,
        };
        let renderer = Renderer {
            vao: 0,
            node_count: 0,
            shader: Shader::with_geometry_shader(
                "assets/shaders/octree/visualize.vert.glsl",
                "assets/shaders/octree/visualize.frag.glsl",
                "assets/shaders/octree/visualize.geom.glsl",
            ),
            bricks_shader: Shader::with_geometry_shader(
                "assets/shaders/octree/visualizeBricks.vert.glsl",
                "assets/shaders/octree/visualizeBricks.frag.glsl",
                "assets/shaders/octree/visualizeBricks.geom.glsl",
            ),
            bricks_to_show: BricksToShow::default(),
            node_positions_shader: Shader::with_geometry_shader(
                "assets/shaders/debug/nodePositions.vert.glsl",
                "assets/shaders/debug/nodePositions.frag.glsl",
                "assets/shaders/debug/nodePositions.geom.glsl",
            ),
            node_neighbors_shader: Shader::with_geometry_shader(
                "assets/shaders/debug/nodeNeighbors.vert.glsl",
                "assets/shaders/debug/nodeNeighbors.frag.glsl",
                "assets/shaders/debug/nodeNeighbors.geom.glsl",
            ),
            node_bricks_shader: Shader::with_geometry_shader(
                "assets/shaders/debug/nodeBricks.vert.glsl",
                "assets/shaders/debug/nodeBricks.frag.glsl",
                "assets/shaders/debug/nodeBricks.geom.glsl",
            ),
        };

        let mut octree = Self {
            textures,
            nodes_per_level,
            voxel_data,
            renderer,
        };

        octree.build();

        octree
    }

    fn get_max_node_pool_size() -> usize {
        let number_of_nodes = (0..CONFIG.octree_levels)
            .map(|exponent| 8_usize.pow(exponent))
            .sum::<usize>();
        number_of_nodes * constants::CHILDREN_PER_NODE as usize
    }

    unsafe fn initialize_textures(max_node_pool_size: usize) -> OctreeTextures {
        OctreeTextures {
            node_pool: helpers::generate_texture_buffer(max_node_pool_size, gl::R32UI, 0u32),
            brick_pointers: helpers::generate_texture_buffer(max_node_pool_size, gl::R32UI, 0u32),
            node_positions: helpers::generate_texture_buffer(max_node_pool_size, gl::R32UI, 0u32),
            neighbors: [
                helpers::generate_texture_buffer(max_node_pool_size, gl::R32UI, 0u32), // X
                helpers::generate_texture_buffer(max_node_pool_size, gl::R32UI, 0u32), // -X
                helpers::generate_texture_buffer(max_node_pool_size, gl::R32UI, 0u32), // Y
                helpers::generate_texture_buffer(max_node_pool_size, gl::R32UI, 0u32), // -Y
                helpers::generate_texture_buffer(max_node_pool_size, gl::R32UI, 0u32), // Z
                helpers::generate_texture_buffer(max_node_pool_size, gl::R32UI, 0u32), // -Z
            ],
            level_start_indices: helpers::generate_texture_buffer(
                (CONFIG.octree_levels + 1) as usize,
                gl::R32UI,
                0u32,
            ),
            brick_pool_colors: helpers::generate_3d_rgba_texture(CONFIG.brick_pool_resolution),
            brick_pool_normals: helpers::generate_3d_rgba_texture(CONFIG.brick_pool_resolution),
            brick_pool_photons: helpers::generate_3d_r32ui_texture(CONFIG.brick_pool_resolution),
        }
    }

    #[allow(dead_code)]
    pub unsafe fn show_nodes(&self, offset: usize, number_of_nodes: usize) {
        let max_node_pool_size = Self::get_max_node_pool_size();

        let values = vec![1u32; max_node_pool_size];
        gl::BindBuffer(gl::TEXTURE_BUFFER, self.textures.node_pool.1);
        gl::GetBufferSubData(
            gl::TEXTURE_BUFFER,
            0,
            (size_of::<GLuint>() * max_node_pool_size) as isize,
            values.as_ptr() as *mut c_void,
        );

        for node in 0..number_of_nodes {
            let lower_limit: usize = (node + offset) * constants::CHILDREN_PER_NODE as usize;
            let upper_limit: usize = lower_limit + constants::CHILDREN_PER_NODE as usize;
            debug!("{:?}", &values[lower_limit..upper_limit]);
        }
    }
}
