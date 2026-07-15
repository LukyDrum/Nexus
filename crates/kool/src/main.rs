use std::path::PathBuf;

use clap::Parser;
use kool::{ElementalArgs, ElementalConfig, ElementalWidget, KoolWidgetRunner, scan_and_parse};

fn main() {
    let args = ElementalArgs::parse();

    let mut config_path = PathBuf::from(args.config);
    let config = std::fs::read_to_string(&config_path).expect("Failed to read config file.");
    let config: ElementalConfig = toml::from_str(&config).expect("Failed to parse config file.");

    config_path.pop();
    std::env::set_current_dir(config_path).expect("Failed to change current working directory.");

    let root = scan_and_parse(&config.kool).expect("Failed to parse kool.");

    let widget = ElementalWidget {
        name: config.name,
        settings: config.settings,
        style: config.visual,
        root,
    };

    KoolWidgetRunner::run(widget).expect("Failed to run widget.");
}
