use std::{ffi::c_void, mem::size_of};

use gl::types::GLuint;
use log;
use serde::{Serialize, Deserialize};
use engine::prelude::*;

use crate::{
    config::Config,
    constants,
};

mod build;
mod lighting;
mod visualize;
mod voxel_data;

use build::*;
pub use visualize::{BrickAttribute, BricksToShow};

use lighting::{PhotonsToIrradiance, StorePhotons, ClearLight, LightTransfer};
pub use voxel_data::VoxelData;

pub struct Octree {
    pub geometry_data: OctreeData,
    pub border_data: OctreeData,
    pub textures: OctreeTextures,
    renderer: Renderer,
    builder: Builder,
}

pub struct OctreeTextures {
    pub node_pool: BufferTexture,
    brick_pointers: BufferTexture,
    pub node_positions: BufferTexture,
    neighbors: [BufferTexture; 6],
    pub brick_pool_colors_raw: Texture3D, // Raw colors, they are then moved to `brick_pool_colors`
    pub brick_pool_colors: [Texture3D; 6], // Anisotropic voxels, one texture per main direction
    pub brick_pool_alpha: Texture3D,
    pub brick_pool_irradiance: [Texture3D; 6], // Anisotropic voxels
    pub brick_pool_normals: Texture3D,
    pub brick_pool_photons: Texture3D,
    pub photons_buffer: BufferTexture,
    pub children_buffer: BufferTexture,
    pub color_quad_textures: [Texture2D; 2],
}

pub struct OctreeData {
    pub node_data: NodeData,
    pub voxel_data: VoxelData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OctreeDataType {
    Geometry,
    Border,
}

impl OctreeDataType {
    pub fn next(&self) -> Self {
        use OctreeDataType::*;
        match self {
            Geometry => Border,
            Border => Geometry,
        }
    }
}

impl Default for OctreeDataType {
    fn default() -> Self {
        Self::Geometry
    }
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

struct Renderer {
    vao: GLuint,
    node_count: u32,
    shader: Shader,
    normals_shader: Shader,
    bricks_shader: Shader,
    bricks_to_show: BricksToShow,
    node_positions_shader: Shader,
    node_neighbors_shader: Shader,
    node_bricks_shader: Shader,
    get_photons_shader: Shader,
    get_children_shader: Shader,
    eye_ray_shader: Shader,
    get_colors_quad_shader: Shader,
}

struct Builder {
    neighbor_pointers_pass: NeighborPointersPass,
    flag_nodes_pass: FlagNodesPass,
    allocate_nodes_pass: AllocateNodesPass,
    store_node_positions_pass: StoreNodePositions,
    write_leaf_nodes_pass: WriteLeafNodesPass,
    spread_leaf_bricks_pass: SpreadLeafBricksPass,
    leaf_border_transfer_pass: LeafBorderTransferPass,
    anisotropic_border_transfer_pass: AnisotropicBorderTransferPass,
    mipmap_anisotropic_pass: MipmapAnisotropicPass,
    mipmap_isotropic_pass: MipmapIsotropicPass,
    append_border_voxel_fragments_pass: AppendBorderVoxelFragmentsPass,
    photons_to_irradiance_pass: PhotonsToIrradiance,
    process_raw_brick_pool_colors: ProcessRawBrickPoolColors,
    create_alpha_map: CreateAlphaMap,
    store_photons: StorePhotons,
    clear_light: ClearLight,
    light_transfer: LightTransfer,
}

impl Octree {
    /// Creates a Sparse Voxel Octree (SVO)
    pub unsafe fn new(
        voxel_positions: BufferTextureV2<u32>,
        number_of_voxel_fragments: u32,
        voxel_colors: BufferTexture,
        voxel_normals: BufferTexture,
    ) -> Self {
        let config = Config::instance();
        let max_node_pool_size = Self::get_max_node_pool_size();
        let max_node_pool_size_in_bytes = size_of::<GLuint>() * max_node_pool_size as usize;
        let textures = Self::initialize_textures(max_node_pool_size_in_bytes);
        let geometry_data = OctreeData {
            node_data: NodeData {
                nodes_per_level: Vec::new(),
                level_start_indices: helpers::generate_texture_buffer(
                    (config.octree_levels() + 1) as usize,
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
                    (config.octree_levels() + 1) as usize,
                    gl::R32UI,
                    0u32,
                ),
            },
            voxel_data: VoxelData {
                voxel_positions: BufferTextureV2::from_data(
                    vec![0u32; number_of_voxel_fragments as usize], // TODO: Should be smaller
                ),
                number_of_voxel_fragments: 0, // Will be initialized empty later
                voxel_colors: (0, 0),         // Will be initialized empty later
                voxel_normals: (0, 0),        // Will be initialized empty later
            },
        };
        let octree_renderer = Renderer {
            vao: 0,
            node_count: 0,
            shader: compile_shaders!(
                "assets/shaders/octree/visualize.vert.glsl",
                "assets/shaders/octree/visualize.frag.glsl",
                "assets/shaders/octree/visualize.geom.glsl",
            ),
            normals_shader: compile_shaders!(
                "assets/shaders/octree/visualizeBrickNormals.vert.glsl",
                "assets/shaders/octree/visualizeBrickNormals.frag.glsl",
                "assets/shaders/octree/visualizeBrickNormals.geom.glsl",
            ),
            bricks_shader: compile_shaders!(
                "assets/shaders/octree/visualizeBricks.vert.glsl",
                "assets/shaders/octree/visualizeBricks.frag.glsl",
                "assets/shaders/octree/visualizeBricks.geom.glsl",
            ),
            bricks_to_show: BricksToShow::default(),
            node_positions_shader: compile_shaders!(
                "assets/shaders/debug/nodePositions.vert.glsl",
                "assets/shaders/debug/nodePositions.frag.glsl",
                "assets/shaders/debug/nodePositions.geom.glsl",
            ),
            node_neighbors_shader: compile_shaders!(
                "assets/shaders/debug/nodeNeighbors.vert.glsl",
                "assets/shaders/debug/nodeNeighbors.frag.glsl",
                "assets/shaders/debug/nodeNeighbors.geom.glsl",
            ),
            node_bricks_shader: compile_shaders!(
                "assets/shaders/debug/nodeBricks.vert.glsl",
                "assets/shaders/debug/nodeBricks.frag.glsl",
                "assets/shaders/debug/nodeBricks.geom.glsl",
            ),
            get_photons_shader: compile_compute!("assets/shaders/debug/getPhotons.comp.glsl"),
            get_children_shader: compile_compute!("assets/shaders/debug/getChildren.comp.glsl"),
            eye_ray_shader: compile_shaders!("assets/shaders/debug/eyeRay.glsl"),
            get_colors_quad_shader: compile_shaders!(
                "assets/shaders/debug/debugInterpolation.glsl",
            ),
        };
        let builder = Builder {
            neighbor_pointers_pass: NeighborPointersPass::init(),
            flag_nodes_pass: FlagNodesPass::init(),
            allocate_nodes_pass: AllocateNodesPass::init(),
            store_node_positions_pass: StoreNodePositions::init(),
            write_leaf_nodes_pass: WriteLeafNodesPass::init(),
            spread_leaf_bricks_pass: SpreadLeafBricksPass::init(),
            leaf_border_transfer_pass: LeafBorderTransferPass::init(),
            anisotropic_border_transfer_pass: AnisotropicBorderTransferPass::init(),
            mipmap_anisotropic_pass: MipmapAnisotropicPass::init(),
            mipmap_isotropic_pass: MipmapIsotropicPass::init(),
            append_border_voxel_fragments_pass: AppendBorderVoxelFragmentsPass::init(),
            photons_to_irradiance_pass: PhotonsToIrradiance::init(),
            process_raw_brick_pool_colors: ProcessRawBrickPoolColors::init(),
            create_alpha_map: CreateAlphaMap::init(),
            store_photons: StorePhotons::init(),
            clear_light: ClearLight::init(),
            light_transfer: LightTransfer::init(),
        };

        let mut octree = Self {
            geometry_data,
            border_data,
            textures,
            renderer: octree_renderer,
            builder,
        };

        octree.build();

        octree
    }

    unsafe fn get_max_node_pool_size() -> usize {
        let config = Config::instance();
        let number_of_nodes = (0..config.octree_levels())
            .map(|exponent| 8_usize.pow(exponent))
            .sum::<usize>();
        let max_node_pool_size = number_of_nodes * constants::CHILDREN_PER_NODE as usize;
        log::debug!(
            "Max node pool size based on tree height: {}",
            max_node_pool_size
        );

        let mut max_texture_buffer_size = 0; // In bytes
        gl::GetIntegerv(gl::MAX_TEXTURE_BUFFER_SIZE, &mut max_texture_buffer_size);
        max_texture_buffer_size /= 8;

        let max_node_pool_size =
            max_node_pool_size.min((max_texture_buffer_size).try_into().unwrap());
        log::debug!(
            "Max node pool size based on memory max: {}",
            max_texture_buffer_size
        );
        log::debug!(
            "Final node pool size based on tree height: {}",
            max_node_pool_size
        );

        max_node_pool_size
    }

    unsafe fn initialize_textures(max_node_pool_size: usize) -> OctreeTextures {
        let config = Config::instance();
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
            brick_pool_colors_raw: helpers::generate_3d_r32ui_texture(config.brick_pool_resolution),
            brick_pool_colors: [
                helpers::generate_3d_rgba_texture(config.brick_pool_resolution), // (X, +), also used for lower level
                helpers::generate_3d_rgba_texture(config.brick_pool_resolution), // (X, -)
                helpers::generate_3d_rgba_texture(config.brick_pool_resolution), // (Y, +)
                helpers::generate_3d_rgba_texture(config.brick_pool_resolution), // (Y, -)
                helpers::generate_3d_rgba_texture(config.brick_pool_resolution), // (Z, +)
                helpers::generate_3d_rgba_texture(config.brick_pool_resolution), // (Z, -)
            ],
            brick_pool_alpha: helpers::generate_3d_rgba_texture(config.brick_pool_resolution),
            brick_pool_irradiance: [
                helpers::generate_3d_rgba_texture(config.brick_pool_resolution), // (X, +), also used for lower level
                helpers::generate_3d_rgba_texture(config.brick_pool_resolution), // (X, -)
                helpers::generate_3d_rgba_texture(config.brick_pool_resolution), // (Y, +)
                helpers::generate_3d_rgba_texture(config.brick_pool_resolution), // (Y, -)
                helpers::generate_3d_rgba_texture(config.brick_pool_resolution), // (Z, +)
                helpers::generate_3d_rgba_texture(config.brick_pool_resolution), // (Z, -)
            ],
            brick_pool_normals: helpers::generate_3d_rgba32f_texture(config.brick_pool_resolution),
            brick_pool_photons: helpers::generate_3d_r32ui_texture(config.brick_pool_resolution),
            photons_buffer: helpers::generate_texture_buffer(27, gl::R32UI, 0u32), // 27 voxels in a brick
            children_buffer: helpers::generate_texture_buffer(8, gl::R32UI, 0_u32), // 8 children in a node
            color_quad_textures: {
                let mut textures = [0; 2];

                let (viewport_width, viewport_height) = config.viewport_dimensions();

                gl::GenTextures(2, textures.as_mut_ptr());
                gl::BindTexture(gl::TEXTURE_2D, textures[0]);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA8 as i32,
                    viewport_width,
                    viewport_height,
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
                    viewport_width,
                    viewport_height,
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
            log::debug!("{}: {:?}", node, &values[lower_limit..upper_limit]);
        }
    }
}
