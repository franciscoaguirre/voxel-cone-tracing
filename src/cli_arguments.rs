use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Options")]
pub struct Options {
    /// Activate debug messages
    #[structopt(long)]
    pub debug: bool,

    /// Model to use
    #[structopt(long, default_value = "triangle.obj")]
    pub model: String,
}
