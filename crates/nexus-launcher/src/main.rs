mod args;
mod config;
mod desktop;
mod history;
mod launcher;

use std::path::PathBuf;

use clap::Parser;
use nexus_widgets::NexusWidgetRunner;

use crate::{
    args::LauncherArgs, config::LauncherConfig, history::History, launcher::NexusLauncher,
};

fn main() {
    let args = LauncherArgs::parse();

    let config = args
        .config
        .map_or_else(LauncherConfig::default, read_config);

    let history_path = args
        .history
        .map_or_else(History::default_path, PathBuf::from);
    let history = History::read_from(history_path).unwrap_or_default();

    let launcher = NexusLauncher::new(config, history);
    NexusWidgetRunner::run(launcher).unwrap()
}

fn read_config(path: String) -> LauncherConfig {
    let content = std::fs::read_to_string(path).expect("Failed to read config file.");
    toml::from_str(&content).expect("Failed to parse config file.")
}
