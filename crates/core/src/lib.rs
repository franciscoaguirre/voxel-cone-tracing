//! Core module to run Voxel Cone Tracing.
//! It contains the main algorithms:
//! - Voxelization
//! - SVO construction, filtering and updating
//! - Proper cone tracing
//! A UI is provided to interact with the different algorithms and tweak parameters.

pub mod cone_tracing;
pub mod menu;
pub mod octree;
pub mod voxelization;

pub mod config;
mod constants;
