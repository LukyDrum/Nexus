use crate::style::CommonStyle;

/// Describes how a `CommonStyle` applies to an iced element.
pub trait WithStyle {
    fn with_style(self, style: CommonStyle) -> Self;
}

/* `WithStyle` impls for iced elements */

impl<'a> WithStyle for iced::widget::Text<'a> {
    fn with_style(self, style: CommonStyle) -> Self {
        self.width(style.width())
            .height(style.height())
            .size(style.text_size())
            .line_height(style.line_height())
            .align_x(style.align_x())
            .align_y(style.align_y())
            .font_maybe(style.font())
            .style(move |_theme| iced::widget::text::Style {
                color: style.color(),
            })
    }
}

impl<'a, Msg> WithStyle for iced::widget::Container<'a, Msg> {
    fn with_style(self, style: CommonStyle) -> Self {
        self.padding(style.padding())
            .width(style.width())
            .height(style.height())
            .align_x(style.align_x())
            .align_y(style.align_y())
            .style(move |_theme| iced::widget::container::Style {
                text_color: style.color(),
                background: style.background(),
                border: style.border(),
                ..Default::default()
            })
    }
}

impl<'a, Msg> WithStyle for iced::widget::Button<'a, Msg> {
    fn with_style(self, style: CommonStyle) -> Self {
        self.padding(style.padding())
            .width(style.width())
            .height(style.height())
            .style(move |theme, _| iced::widget::button::Style {
                background: style.background(),
                text_color: style.color().unwrap_or_else(|| theme.palette().text),
                border: style.border(),
                ..Default::default()
            })
    }
}

impl<'a, Msg> WithStyle for iced::widget::TextInput<'a, Msg>
where
    Msg: Clone,
{
    fn with_style(self, style: CommonStyle) -> Self {
        self.padding(style.padding())
            .width(style.width())
            .size(style.text_size())
            .line_height(style.line_height())
            .align_x(style.align_x())
            .font(style.font().unwrap_or_default())
            .style(move |theme, _| {
                let palette = theme.palette();

                iced::widget::text_input::Style {
                    background: style.background().unwrap_or(palette.background.into()),
                    border: style.border(),
                    icon: style.color().unwrap_or(palette.text),
                    placeholder: style.color().unwrap_or(palette.text).scale_alpha(0.6),
                    value: style.color().unwrap_or(palette.text),
                    selection: style.highlight().unwrap_or(palette.primary),
                }
            })
    }
}

impl<'a, Msg> WithStyle for iced::widget::Scrollable<'a, Msg> {
    fn with_style(self, style: CommonStyle) -> Self {
        self.width(style.width())
            .height(style.height())
            .style(move |theme, status| {
                let mut scrollable = iced::widget::scrollable::default(theme, status);

                scrollable.container = iced::widget::container::Style {
                    text_color: style.color(),
                    background: style.background(),
                    border: style.border(),
                    ..Default::default()
                };
                scrollable.horizontal_rail = scrollable.horizontal_rail.with_style(style);
                scrollable.vertical_rail = scrollable.vertical_rail.with_style(style);
                scrollable.gap = style.background();

                scrollable
            })
    }
}

impl WithStyle for iced::widget::scrollable::Rail {
    fn with_style(mut self, style: CommonStyle) -> Self {
        self.background = style.background();
        self.border = style.border();
        self.scroller = self.scroller.with_style(style);

        self
    }
}

impl WithStyle for iced::widget::scrollable::Scroller {
    fn with_style(mut self, style: CommonStyle) -> Self {
        self.background = style.background().unwrap_or(self.background);
        self.border = style.border();

        self
    }
}

impl WithStyle for iced::widget::Image {
    fn with_style(self, style: CommonStyle) -> Self {
        self.border_radius(style.border().radius)
            .width(style.width())
            .height(style.height())
            .opacity(style.opacity())
    }
}

impl<'a, Msg> WithStyle for iced::widget::Column<'a, Msg> {
    fn with_style(self, style: CommonStyle) -> Self {
        self.align_x(style.align_x())
            .width(style.width())
            .height(style.height())
            .padding(style.padding())
            .spacing(style.spacing())
    }
}

impl<'a, Msg> WithStyle for iced::widget::Row<'a, Msg> {
    fn with_style(self, style: CommonStyle) -> Self {
        self.align_y(style.align_y())
            .width(style.width())
            .height(style.height())
            .padding(style.padding())
            .spacing(style.spacing())
    }
}

impl<'a, Msg> WithStyle for iced::widget::Stack<'a, Msg> {
    fn with_style(self, style: CommonStyle) -> Self {
        self.width(style.width()).height(style.height())
    }
}
