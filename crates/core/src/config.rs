use once_cell::sync::OnceCell;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_brick_pool_resolution")]
    pub brick_pool_resolution: u32,
    // TODO: This could be different than the one in the shaders right now
    #[serde(default = "default_working_group_size")]
    pub working_group_size: u32,
    #[serde(default = "default_viewport_dimensions")]
    viewport_dimensions: (i32, i32),
    voxel_dimension: u32,
    #[serde(skip_deserializing)]
    octree_levels: u32,
    #[serde(skip_deserializing)]
    last_octree_level: u32,
}

const fn default_brick_pool_resolution() -> u32 {
    384
}

const fn default_working_group_size() -> u32 {
    64
}

const fn default_viewport_dimensions() -> (i32, i32) {
    (840, 840)
}

static mut INSTANCE: OnceCell<Config> = OnceCell::new();

impl Config {
    /// Get the single instance of this struct
    pub fn instance() -> &'static Self {
        unsafe {
            if let Some(config) = INSTANCE.get() {
                &config
            } else {
                panic!("Must initialize core config");
            }
        }
    }

    /// Initializes the config
    /// Must be called before any call to `instance`
    /// Overrides any previous config
    pub unsafe fn initialize(mut config: Self) {
        config.set_voxel_dimension(config.voxel_dimension);
        let _ = INSTANCE.set(config);
    }

    pub fn new(voxel_dimension_exponent: u32) -> Self {
        Self {
            brick_pool_resolution: 384,
            working_group_size: 64,
            viewport_dimensions: (840, 840),
            // Will be set correctly in `initialize` later
            voxel_dimension: voxel_dimension_exponent,
            octree_levels: 0,
            last_octree_level: 0,
        }
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

    // TODO: Fix, the `should_panic` is not working
    #[test]
    #[should_panic]
    // Given this is stateful code, this test should run first (they are ran alfabetically)
    fn aauninitialized_config_should_panic() {
        let _ = Config::instance();
    }

    #[test]
    fn config_initialization_works() {
        let voxel_dimension_exponent = 4;
        unsafe {
            Config::initialize(Config::new(voxel_dimension_exponent));
        };
        let config = Config::instance();
        assert_eq!(config.voxel_dimension(), 16);
        assert_eq!(config.octree_levels(), 4);
        assert_eq!(config.last_octree_level(), 3);
    }

    #[test]
    #[should_panic]
    fn initializing_twice_should_panic() {
        let voxel_dimension_exponent = 4;
        unsafe {
            Config::initialize(Config::new(voxel_dimension_exponent));
        };
    }
}
