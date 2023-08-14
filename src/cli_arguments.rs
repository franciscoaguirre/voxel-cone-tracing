use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Options")]
pub struct Options {
    /// Model to use
    #[structopt(long, default_value = "triangle")]
    pub model: String,

    /// Scene file to load
    #[structopt(long, default_value = "scene")]
    pub scene: String,
}
