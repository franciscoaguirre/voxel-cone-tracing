use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub brick_pool_resolution: u32,
    // TODO: This could be different than the one in the shaders right now
    pub working_group_size: u32,
    viewport_dimensions: (i32, i32),
    voxel_dimension: u32,
    #[serde(skip_deserializing)]
    octree_levels: u32,
    #[serde(skip_deserializing)]
    last_octree_level: u32,
}

use once_cell::sync::OnceCell;

static mut INSTANCE: OnceCell<Config> = OnceCell::new();

impl Config {
    /// Get the single instance of this struct
    pub fn instance() -> &'static Self {
        unsafe {
            if let Some(config) = INSTANCE.get() {
                &config
            } else { panic!("Must initialize core config"); }
        }
    }

    /// Initializes the config
    /// Must be called before any call to `instance`
    pub unsafe fn initialize(mut config: Self) {
        if INSTANCE.get().is_some() { panic!("Can only initialize core config once"); }
        config.set_voxel_dimension(config.voxel_dimension);
        let _ = INSTANCE.set(config);
    }

    pub fn voxel_dimension(&self) -> u32 {
        self.voxel_dimension
    }

    pub fn octree_levels(&self) -> u32 {
        self.octree_levels
    }
    
    pub fn last_octree_level(&self) -> u32 {
        self.last_octree_level
    }

    pub fn viewport_dimensions(&self) -> (i32, i32) {
        self.viewport_dimensions
    }

    /// Sets the voxel dimension, the number of octree levels is based on that
    fn set_voxel_dimension(&mut self, voxel_dimension_exponent: u32) {
        self.voxel_dimension = 2_u32.pow(voxel_dimension_exponent);
        self.octree_levels = self.voxel_dimension.pow(3).ilog2() / 8_u32.ilog2();
        self.last_octree_level = self.octree_levels - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn uninitialized_config_should_panic() {
        let _ = Config::instance();
    }
}
