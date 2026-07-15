#[derive(Debug, clap::Parser)]
pub struct ElementalArgs {
    #[arg(short, long, help = "Path to a config file in .toml format.")]
    pub config: String,
}
