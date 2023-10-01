//! Core module to run Voxel Cone Tracing.
//! It contains the main algorithms:
//! - Voxelization
//! - SVO construction, filtering and updating
//! - Proper cone tracing
//! A UI is provided to interact with the different algorithms and tweak parameters.

pub mod voxelization;
pub mod octree;
pub mod cone_tracing;
pub mod menu;

mod constants;
pub mod config;
