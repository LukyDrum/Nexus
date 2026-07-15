use kool::{ElementalWidget, KoolWidgetRunner, scan_and_parse, style::WidgetStyle};

fn main() {
    let input = r#"Container(key = "outer", Container(Text(key = "textkey", "Ho World!")))"#;
    let element = scan_and_parse(input).unwrap();

    let elemental = ElementalWidget {
        name: "Kool".to_owned(),
        root: element,
        style: WidgetStyle::default_dark(),
    };

    KoolWidgetRunner::run(elemental).unwrap();
}
