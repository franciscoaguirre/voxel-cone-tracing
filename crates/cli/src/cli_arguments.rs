use structopt::StructOpt;

/// Command-line arguments we can pass in to the binary
#[derive(Debug, StructOpt)]
#[structopt(name = "Options")]
pub struct Options {
    /// Scene file to load
    #[structopt(long, default_value = "scene")]
    pub scene: String,

    /// Preset file to load
    #[structopt(long, default_value = "")]
    pub preset: String,

    /// Whether or not to run in "visual tests" mode
    /// This mode runs only once, with a headless window, and
    /// saves all the images to a folder.
    #[structopt(long)]
    pub visual_tests: bool,

    /// Whether or not the screenshot from visual tests should be upgraded
    /// with this run.
    /// By default, visual tests panic if the image is not the same.
    /// This overrides the image with the one generated this run.
    #[structopt(long)]
    pub update_screenshots: bool,
}
