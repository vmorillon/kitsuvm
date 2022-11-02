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

    /// Relative path to template files
    #[arg(required = true)]
    pub templates: Vec<String>,
}
