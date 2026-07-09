use std::rc::Rc;

use crate::style::{ElementWithStyleId, StyleTree, WithStyleId};

pub trait WithStyle {
    const BASE_KEY: &'static str;

    fn with_style(self, style: Rc<StyleTree>) -> Self;

    fn base_tree(style: &Rc<StyleTree>) -> Rc<StyleTree> {
        StyleTree::subtree(style, Self::BASE_KEY)
    }
}

impl<T> WithStyle for ElementWithStyleId<T>
where
    T: WithStyle,
{
    const BASE_KEY: &'static str = T::BASE_KEY;

    fn with_style(self, style: Rc<StyleTree>) -> Self {
        let style_id = self.style_id().clone();
        let id_tree = StyleTree::subtree(&style, style_id.clone());
        let base_tree = Self::base_tree(&style);

        let mut adhoc_tree = StyleTree::default();
        adhoc_tree.set_style(base_tree.style());
        adhoc_tree.nest(Self::BASE_KEY, id_tree);

        // Need to go back to `ElementWithStyleId` due to trait definition
        self.element()
            .with_style(Rc::new(adhoc_tree))
            .with_style_id(style_id)
    }
}

/* `WithStyle` impls for iced elements */

impl<'a> WithStyle for iced::widget::Text<'a> {
    const BASE_KEY: &'static str = "text";

    fn with_style(self, style: Rc<StyleTree>) -> Self {
        let common = style.get(Self::BASE_KEY);

        self.width(common.width())
            .height(common.height())
            .size(common.text_size())
            .line_height(common.line_height())
            .style(move |_theme| iced::widget::text::Style {
                color: common.color(),
            })
    }
}

impl<'a, Msg> WithStyle for iced::widget::Container<'a, Msg> {
    const BASE_KEY: &'static str = "container";

    fn with_style(self, style: Rc<StyleTree>) -> Self {
        let common = style.get(Self::BASE_KEY);

        self.padding(common.padding())
            .width(common.width())
            .height(common.height())
            .style(move |_theme| iced::widget::container::Style {
                text_color: common.color(),
                background: common.background(),
                border: common.border(),
                ..Default::default()
            })
    }
}

impl<'a, Msg> WithStyle for iced::widget::Button<'a, Msg> {
    const BASE_KEY: &'static str = "button";

    fn with_style(self, style: Rc<StyleTree>) -> Self {
        let base_tree = Self::base_tree(&style);
        let common = base_tree.style();

        self.padding(base_tree.style().padding())
            .width(common.width())
            .height(common.height())
            .style(move |theme, status| {
                let common = base_tree.get(status);

                iced::widget::button::Style {
                    background: common.background(),
                    text_color: common.color().unwrap_or(theme.palette().text),
                    border: common.border(),
                    ..Default::default()
                }
            })
    }
}

impl<'a, Msg> WithStyle for iced::widget::TextInput<'a, Msg>
where
    Msg: Clone,
{
    const BASE_KEY: &'static str = "input";

    fn with_style(self, style: Rc<StyleTree>) -> Self {
        let base_tree = Self::base_tree(&style);
        let common = base_tree.style();

        self.padding(base_tree.style().padding())
            .width(common.width())
            .size(common.text_size())
            .line_height(common.line_height())
            .style(move |theme, status| {
                let common = base_tree.get(status);
                let palette = theme.palette();

                iced::widget::text_input::Style {
                    background: common.background().unwrap_or(palette.background.into()),
                    border: common.border(),
                    icon: common.color().unwrap_or(palette.text),
                    placeholder: common.color().unwrap_or(palette.text).scale_alpha(0.6),
                    value: common.color().unwrap_or(palette.text),
                    selection: common.highlight().unwrap_or(palette.primary),
                }
            })
    }
}

impl<'a, Msg> WithStyle for iced::widget::Scrollable<'a, Msg> {
    const BASE_KEY: &'static str = "scroll";

    fn with_style(self, style: Rc<StyleTree>) -> Self {
        let base_tree = Self::base_tree(&style);
        let common = base_tree.style();

        self.width(common.width())
            .height(common.height())
            .style(move |theme, status| {
                let status_tree = StyleTree::subtree(&base_tree, status);
                let common = status_tree.style();
                let gap = status_tree.get("gap");

                let mut scrollable = iced::widget::scrollable::default(theme, status);

                scrollable.container = iced::widget::container::Style {
                    text_color: common.color(),
                    background: common.background(),
                    border: common.border(),
                    ..Default::default()
                };
                scrollable.horizontal_rail =
                    scrollable.horizontal_rail.with_style(status_tree.clone());
                scrollable.vertical_rail = scrollable.vertical_rail.with_style(status_tree);
                scrollable.gap = gap.background();

                scrollable
            })
    }
}

impl WithStyle for iced::widget::scrollable::Rail {
    const BASE_KEY: &'static str = "rail";

    fn with_style(mut self, style: Rc<StyleTree>) -> Self {
        let base_tree = Self::base_tree(&style);
        let common = base_tree.style();

        self.background = common.background();
        self.border = common.border();
        self.scroller = self.scroller.with_style(base_tree);

        self
    }
}

impl WithStyle for iced::widget::scrollable::Scroller {
    const BASE_KEY: &'static str = "scroller";

    fn with_style(mut self, style: Rc<StyleTree>) -> Self {
        let base_tree = Self::base_tree(&style);
        let common = base_tree.style();

        self.background = common.background().unwrap_or(self.background);
        self.border = common.border();

        self
    }
}

impl WithStyle for iced::widget::Image {
    const BASE_KEY: &'static str = "image";

    fn with_style(self, style: Rc<StyleTree>) -> Self {
        let common = style.get(Self::BASE_KEY);

        self.border_radius(common.border().radius)
            .width(common.width())
            .height(common.height())
            .opacity(common.opacity())
    }
}
