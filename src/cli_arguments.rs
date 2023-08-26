use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Options")]
pub struct Options {
    /// Scene file to load
    #[structopt(long, default_value = "scene")]
    pub scene: String,

    /// Preset file to load
    #[structopt(long, default_value = "")]
    pub preset: String,
}
