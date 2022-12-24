use gl::types::*;

/// Variables here are of the form (texture, texture_buffer)
pub static mut OCTREE_NODE_POOL: (GLuint, GLuint) = (0, 0);
pub static mut OCTREE_NODE_POOL_BRICK_POINTERS: (GLuint, GLuint) = (0, 0);
pub static mut OCTREE_NODE_POOL_NEIGHBOUR_X: (GLuint, GLuint) = (0, 0);
pub static mut OCTREE_NODE_POOL_NEIGHBOUR_X_NEGATIVE: (GLuint, GLuint) = (0, 0);
pub static mut OCTREE_NODE_POOL_NEIGHBOUR_Y: (GLuint, GLuint) = (0, 0);
pub static mut OCTREE_NODE_POOL_NEIGHBOUR_Y_NEGATIVE: (GLuint, GLuint) = (0, 0);
pub static mut OCTREE_NODE_POOL_NEIGHBOUR_Z: (GLuint, GLuint) = (0, 0);
pub static mut OCTREE_NODE_POOL_NEIGHBOUR_Z_NEGATIVE: (GLuint, GLuint) = (0, 0);

pub static mut TILES_PER_LEVEL: Vec<u32> = Vec::new();

pub static mut BRICK_POOL_COLORS_TEXTURE: GLuint = 0;
