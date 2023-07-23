use std::fs::File;

use log::info;
use once_cell::sync::Lazy;
use ron::de::from_reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    // Viewport settings
    pub viewport_width: u32,
    pub viewport_height: u32,

    /// Voxel settings
    /// Voxel dimension exponent
    /// Voxel dimension will be 2 to the power of this item
    pub voxel_dimension: u32,
    pub brick_pool_resolution: u32,

    // Octree settings
    #[serde(skip_deserializing)] // Gets calculated based on voxel_dimension
    pub octree_levels: u32, // First level is level 0

    // Compute shader settings
    pub working_group_size: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            viewport_width: 1024,
            viewport_height: 1024,
            voxel_dimension: 256,
            brick_pool_resolution: 384,
            octree_levels: 8,
            working_group_size: 64,
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(load_config);

fn load_config() -> Config {
    let input_path = "config.ron";
    let file = File::open(&input_path).expect("Missing config file!");
    let mut config: Config = from_reader(file).expect("Config file malformed!");
    config.voxel_dimension = 2u32.pow(config.voxel_dimension);
    config.octree_levels = config.voxel_dimension.pow(3).ilog2() / 8_u32.ilog2();
    info!("Configuration used: {:#?}", config);
    config
}
