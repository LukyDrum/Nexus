#[derive(Debug, clap::Parser)]
pub(super) struct LauncherArgs {
    #[arg(short, long, help = "Path to a config file in .toml format.")]
    pub config: Option<String>,
    #[arg(long, help = "Path to a history file.")]
    pub history: Option<String>,
}
