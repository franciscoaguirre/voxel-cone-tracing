use structopt::StructOpt;

/// Command-line arguments we can pass in to the binary
#[derive(Debug, StructOpt)]
#[structopt(name = "Options")]
pub struct Options {
    /// Config file to load
    #[structopt(long, default_value = "config")]
    pub config: String,

    /// Scene file to load
    #[structopt(long, default_value = "scene")]
    pub scene: String,

    /// Preset file to load
    #[structopt(long, default_value = "")]
    pub preset: String,

    /// Whether or not a screenshot should be taken
    #[structopt(long)]
    pub screenshot: bool,

    /// Count FPS for this many seconds and record the average
    #[structopt(long)]
    pub seconds_for_fps: Option<u32>,

    /// Record how much time it takes to build the octree
    #[structopt(long)]
    pub record_octree_build_time: bool,
}

impl Options {
    pub fn get_name(&self) -> String {
        format!(
            "benchmarks/{}_{}_{}",
            &self.config, &self.scene, &self.preset
        )
    }
}
