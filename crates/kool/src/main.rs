use std::path::PathBuf;

use clap::Parser;
use kool::{
    ElementalArgs, ElementalConfig, ElementalWidget, KoolWidgetRunner, init_signal_channel,
    root_environment, scan_and_parse,
};

#[tokio::main]
async fn main() {
    let args = ElementalArgs::parse();

    let mut config_path = PathBuf::from(args.config);
    let config = std::fs::read_to_string(&config_path).expect("Failed to read config file");
    let config: ElementalConfig = toml::from_str(&config).expect("Failed to parse config file");

    config_path.pop();
    std::env::set_current_dir(config_path).expect("Failed to change current working directory");

    let kool_source =
        std::fs::read_to_string(&config.kool).expect("Failed to read Kool source file");
    let kool_source =
        scan_and_parse(&kool_source).expect("Failed to scan and parse the source code");

    let signal_receiver = init_signal_channel();
    let root_environment = root_environment().into_shared();

    let _ = kool_source
        .execute(&root_environment)
        .expect("Failed to execute Kool source code");

    let widget = ElementalWidget::new(
        config.name,
        config.settings,
        config.visual,
        root_environment,
        signal_receiver,
    );

    KoolWidgetRunner::run(widget).expect("Failed to run widget.");
}
