mod args;
mod config;
mod desktop;
mod launcher;

use clap::Parser;
use nexus_widgets::NexusWidgetRunner;

use crate::{args::LauncherArgs, config::LauncherConfig, launcher::NexusLauncher};

fn main() {
    let args = LauncherArgs::parse();

    let config = args
        .config
        .map_or_else(LauncherConfig::default, read_config);

    let launcher = NexusLauncher::new(config);
    NexusWidgetRunner::run(launcher).unwrap()
}

fn read_config(path: String) -> LauncherConfig {
    let content = std::fs::read_to_string(path).expect("Failed to read config file.");
    toml::from_str(&content).expect("Failed to parse config file.")
}
