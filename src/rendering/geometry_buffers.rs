/// Geometry buffers for deferred shading.
#[derive(Debug)]
pub struct GeometryBuffers {
    /// Unnormalized positions, should be used for calculations.
    raw_positions: u32,
    /// Normalized positions, available only for debug purposes.
    positions: u32,
    colors: u32,
    normals: u32,
}

impl GeometryBuffers {
    pub fn new(raw_positions: u32, positions: u32, normals: u32, colors: u32) -> Self {
        Self {
            raw_positions,
            positions,
            colors,
            normals,
        }
    }

    pub fn raw_positions(&self) -> u32 {
        self.raw_positions
    }

    pub fn positions(&self) -> u32 {
        self.positions
    }

    pub fn colors(&self) -> u32 {
        self.colors
    }

    pub fn normals(&self) -> u32 {
        self.normals
    }
}
