//! Core module to run Voxel Cone Tracing.
//! It contains the main algorithms:
//! - Voxelization
//! - SVO construction, filtering and updating
//! - Proper cone tracing
//! It also contains relevant constants and configuration parameters.

pub mod voxelization;
pub mod octree;
pub mod cone_tracing;

mod constants;
mod config;
