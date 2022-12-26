use std::fs::File;

use once_cell::sync::Lazy;
use ron::de::from_reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    // Viewport settings
    pub viewport_width: i32,
    pub viewport_height: i32,

    // Voxel settings
    pub voxel_dimension: i32,
    pub brick_pool_resolution: u32,

    // Octree settings
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
            brick_pool_resolution: 70 * 3,
            octree_levels: 8,
            working_group_size: 64,
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(load_config);

fn load_config() -> Config {
    let input_path = "config.ron";
    let file = File::open(&input_path).expect("Missing config file!");
    let config: Config = from_reader(file).expect("Config file malformed!");
    config
}
