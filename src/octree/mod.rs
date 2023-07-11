use std::{ffi::c_void, mem::size_of};

use gl::types::GLuint;
use log;

use crate::{
    config::CONFIG,
    constants, helpers,
    rendering::shader::Shader,
    types::{BufferTexture, Texture2D, Texture3D},
};

mod build;
mod lighting;
mod visualize;

pub use visualize::BricksToShow;

pub struct Octree {
    geometry_data: OctreeData,
    pub border_data: OctreeData,
    pub textures: OctreeTextures,
    renderer: Renderer,
}

pub struct OctreeTextures {
    pub node_pool: BufferTexture,
    brick_pointers: BufferTexture,
    pub node_positions: BufferTexture,
    neighbors: [BufferTexture; 6],
    pub brick_pool_colors: [Texture3D; 6], // Anisotropic voxels, one texture per main direction
    brick_pool_normals: Texture3D,
    pub brick_pool_photons: Texture3D,
    pub photons_buffer: BufferTexture,
    pub children_buffer: BufferTexture,
    pub color_quad_textures: [Texture2D; 2],
}

pub struct OctreeData {
    node_data: NodeData,
    pub voxel_data: VoxelData,
}

#[derive(Debug)]
pub enum OctreeDataType {
    Geometry,
    Border,
}

impl OctreeData {
    pub fn number_of_nodes(&self) -> usize {
        self.node_data.number_of_nodes()
    }
}

pub struct NodeData {
    nodes_per_level: Vec<u32>,
    level_start_indices: BufferTexture,
}

impl NodeData {
    pub fn number_of_nodes(&self) -> usize {
        self.nodes_per_level.iter().sum::<u32>() as usize
    }
}

pub struct VoxelData {
    pub voxel_positions: BufferTexture,
    pub number_of_voxel_fragments: u32,
    pub voxel_colors: BufferTexture,
    voxel_normals: BufferTexture,
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
    get_photons_shader: Shader,
    get_children_shader: Shader,
    eye_ray_shader: Shader,
    get_colors_quad_shader: Shader,
    light_view_map_shader: Shader,
    store_photons_shader: Shader,
    clear_bricks_shader: Shader,
}

impl Octree {
    /// Creates a Sparse Voxel Octree (SVO)
    pub unsafe fn new(
        voxel_positions: BufferTexture,
        number_of_voxel_fragments: u32,
        voxel_colors: BufferTexture,
        voxel_normals: BufferTexture,
    ) -> Self {
        let max_node_pool_size = Self::get_max_node_pool_size();
        let max_node_pool_size_in_bytes = size_of::<GLuint>() * max_node_pool_size as usize;
        let textures = Self::initialize_textures(max_node_pool_size_in_bytes);
        let geometry_data = OctreeData {
            node_data: NodeData {
                nodes_per_level: Vec::new(),
                level_start_indices: helpers::generate_texture_buffer(
                    (CONFIG.octree_levels + 1) as usize,
                    gl::R32UI,
                    0u32,
                ),
            },
            voxel_data: VoxelData {
                voxel_positions,
                number_of_voxel_fragments,
                voxel_colors,
                voxel_normals,
            },
        };
        let border_data = OctreeData {
            node_data: NodeData {
                nodes_per_level: Vec::new(),
                level_start_indices: helpers::generate_texture_buffer(
                    (CONFIG.octree_levels + 1) as usize,
                    gl::R32UI,
                    0u32,
                ),
            },
            voxel_data: VoxelData {
                voxel_positions: helpers::generate_texture_buffer(
                    size_of::<GLuint>() * number_of_voxel_fragments as usize, // TODO: Should be smaller
                    gl::R32UI,
                    64u32,
                ),
                number_of_voxel_fragments: 0, // Will be initialized empty later
                voxel_colors: (0, 0),         // Will be initialized empty later
                voxel_normals: (0, 0),        // Will be initialized empty later
            },
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
            get_photons_shader: Shader::new_compute("assets/shaders/debug/getPhotons.comp.glsl"),
            get_children_shader: Shader::new_compute("assets/shaders/debug/getChildren.comp.glsl"),
            eye_ray_shader: Shader::new_single("assets/shaders/debug/eyeRay.glsl"),
            get_colors_quad_shader: Shader::new_single(
                "assets/shaders/debug/debugInterpolation.glsl",
            ),
            light_view_map_shader: Shader::new_single("assets/shaders/octree/lightViewMap.glsl"),
            store_photons_shader: Shader::new_compute(
                "assets/shaders/octree/storePhotons.comp.glsl",
            ),
            clear_bricks_shader: Shader::new_compute("assets/shaders/octree/clearBricks.comp.glsl"),
        };

        let mut octree = Self {
            geometry_data,
            border_data,
            textures,
            renderer,
        };

        octree.build();

        octree
    }

    unsafe fn get_max_node_pool_size() -> usize {
        let number_of_nodes = (0..CONFIG.octree_levels)
            .map(|exponent| 8_usize.pow(exponent))
            .sum::<usize>();
        let max_node_pool_size = number_of_nodes * constants::CHILDREN_PER_NODE as usize;
        log::info!(
            "Max node pool size based on tree height: {}",
            max_node_pool_size
        );

        let mut max_texture_buffer_size = 0; // In bytes
        gl::GetIntegerv(gl::MAX_TEXTURE_BUFFER_SIZE, &mut max_texture_buffer_size);
        max_texture_buffer_size /= 8;

        let max_node_pool_size =
            max_node_pool_size.min((max_texture_buffer_size).try_into().unwrap());
        log::info!(
            "Max node pool size based on memory max: {}",
            max_texture_buffer_size
        );
        log::info!(
            "Final node pool size based on tree height: {}",
            max_node_pool_size
        );

        max_node_pool_size
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
            brick_pool_colors: [
                helpers::generate_3d_rgba_texture(CONFIG.brick_pool_resolution), // (X, +), also used for lower level
                helpers::generate_3d_rgba_texture(CONFIG.brick_pool_resolution), // (X, -)
                helpers::generate_3d_rgba_texture(CONFIG.brick_pool_resolution), // (Y, +)
                helpers::generate_3d_rgba_texture(CONFIG.brick_pool_resolution), // (Y, -)
                helpers::generate_3d_rgba_texture(CONFIG.brick_pool_resolution), // (Z, +)
                helpers::generate_3d_rgba_texture(CONFIG.brick_pool_resolution), // (Z, -)
            ],
            brick_pool_normals: helpers::generate_3d_rgba_texture(CONFIG.brick_pool_resolution),
            brick_pool_photons: helpers::generate_3d_r32ui_texture(CONFIG.brick_pool_resolution),
            photons_buffer: helpers::generate_texture_buffer(27, gl::R32UI, 0u32), // 27 voxels in a brick
            children_buffer: helpers::generate_texture_buffer(8, gl::R32UI, 0_u32), // 8 children in a node
            color_quad_textures: {
                let mut textures = [0; 2];

                gl::GenTextures(2, textures.as_mut_ptr());
                gl::BindTexture(gl::TEXTURE_2D, textures[0]);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA8 as i32,
                    CONFIG.viewport_width as i32,
                    CONFIG.viewport_height as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    std::ptr::null(),
                );
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                gl::BindTexture(gl::TEXTURE_2D, 0);

                gl::BindTexture(gl::TEXTURE_2D, textures[1]);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA8 as i32,
                    CONFIG.viewport_width as i32,
                    CONFIG.viewport_height as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    std::ptr::null(),
                );
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                gl::BindTexture(gl::TEXTURE_2D, 0);

                textures
            },
        }
    }

    pub fn number_of_nodes(&self) -> usize {
        self.geometry_data.number_of_nodes() + self.border_data.number_of_nodes()
    }

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
            log::debug!("{:?}", &values[lower_limit..upper_limit]);
        }
    }
}
