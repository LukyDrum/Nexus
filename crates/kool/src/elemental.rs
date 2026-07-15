use crate::{
    KoolWidget,
    element::{BuildContext, KoolBuilder, KoolElement},
    settings::WidgetSettings,
    style::WidgetStyle,
};

#[derive(Clone, Debug)]
pub struct ElementalWidget {
    pub name: String,
    pub root: KoolElement,
    pub style: WidgetStyle,
}

#[derive(Clone, Debug)]
pub enum ElementalMessage {}

impl KoolWidget<ElementalMessage> for ElementalWidget {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn settings(&self) -> WidgetSettings {
        WidgetSettings::default()
    }

    fn update(&mut self, _message: ElementalMessage) -> iced::Task<ElementalMessage> {
        iced::Task::none()
    }

    fn view(&'_ self) -> impl Into<iced::Element<'_, ElementalMessage>> {
        self.root.build(BuildContext {
            style: self.style.style_tree(),
        })
    }

    fn style(&self) -> &crate::style::WidgetStyle {
        &self.style
    }
}
