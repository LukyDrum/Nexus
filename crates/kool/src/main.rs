use clap::Parser;
use kool::{ElementalArgs, ElementalConfig, ElementalWidget, KoolWidgetRunner, scan_and_parse};

fn main() {
    let args = ElementalArgs::parse();

    let config = std::fs::read_to_string(args.config).expect("Failed to read config file.");
    let config: ElementalConfig = toml::from_str(&config).expect("Failed to parse config file.");

    let root = scan_and_parse(&config.kool).expect("Failed to parse kool.");

    let widget = ElementalWidget {
        name: config.name,
        settings: config.settings,
        style: config.visual,
        root,
    };

    KoolWidgetRunner::run(widget).expect("Failed to run widget.");
}
