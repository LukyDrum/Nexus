use std::path::PathBuf;

use clap::Parser;
use kool::{
    ElementalArgs, ElementalConfig, ElementalWidget, KoolWidgetRunner, ParserContext,
    scan_and_parse_with_context,
};

fn main() {
    let args = ElementalArgs::parse();

    let mut config_path = PathBuf::from(args.config);
    let config = std::fs::read_to_string(&config_path).expect("Failed to read config file.");
    let config: ElementalConfig = toml::from_str(&config).expect("Failed to parse config file.");

    config_path.pop();
    std::env::set_current_dir(config_path).expect("Failed to change current working directory.");

    let mut context = ParserContext::default();
    for pair in args.var {
        context.variables.set(&pair.name, pair.value);
    }

    let root =
        scan_and_parse_with_context(&config.kool, &mut context).expect("Failed to parse kool.");

    let widget = ElementalWidget::new(config.name, config.settings, config.visual, root, context);

    KoolWidgetRunner::run(widget).expect("Failed to run widget.");
}
