use gl::types::*;

pub static mut OCTREE_NODE_POOL_TEXTURE: GLuint = 0;
pub static mut OCTREE_NODE_POOL_TEXTURE_BUFFER: GLuint = 0;

pub static mut OCTREE_NODE_POOL_BRICK_POINTERS_TEXTURE: GLuint = 0;
pub static mut OCTREE_NODE_POOL_BRICK_POINTERS_TEXTURE_BUFFER: GLuint = 0;

pub static mut TILES_PER_LEVEL: Vec<u32> = Vec::new();
