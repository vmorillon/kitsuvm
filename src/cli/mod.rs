use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Relative path to common config file
    #[arg(short, long, default_value = "./project.toml" )]
    pub project: String,
    /// Relative path to instances file
    #[arg(short, long, default_value = "./instances.toml" )]
    pub instances: String,
    /// Relative path to output directory
    #[arg(short, long, default_value = "./out" )]
    pub output: String,
    /// Relative search path to tera template files
    #[arg(short, long, default_value = "templates/**/*.sv.j2" )]
    pub templates: String,

    /// Relative path to vip files
    #[arg(required = true)]
    pub vips: Vec<String>,
}
