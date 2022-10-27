use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Relative path to common config file
    #[arg(short, long, default_value = "./common.toml" )]
    pub common: String,
    /// Relative path to pinlist file
    #[arg(short, long, default_value = "./pinlist.toml" )]
    pub pinlist: String,

    /// Relative path to template files
    #[arg(required = true)]
    pub templates: Vec<String>,
}
