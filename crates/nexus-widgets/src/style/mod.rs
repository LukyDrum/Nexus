use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

mod common;
mod id;
mod key;
mod palette;
mod tree;
mod with;

pub use common::{Border, CommonStyle};
pub use id::{ElementWithStyleId, WithStyleId};
pub use key::{AsStyleKey, StyleKey};
pub use palette::{Color, Palette};
pub use tree::StyleTree;
pub use with::WithStyle;

const WIDGET_KEY: &str = "widget";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct WidgetStyleMediator {
    #[serde(default = "Palette::dark")]
    palette: Palette,
    #[serde(default)]
    style: BTreeMap<StyleKey, CommonStyle>,
}

impl From<WidgetStyleMediator> for WidgetStyle {
    fn from(mut mediator: WidgetStyleMediator) -> Self {
        let widget_style = mediator
            .style
            .remove(&WIDGET_KEY.into())
            .unwrap_or_default();

        let mut tree = StyleTree::default();
        tree.set_style(widget_style);
        for (key, style) in mediator.style {
            tree.insert(key, style);
        }

        WidgetStyle {
            palette: mediator.palette,
            tree,
        }
    }
}

impl From<WidgetStyle> for WidgetStyleMediator {
    fn from(widget_style: WidgetStyle) -> Self {
        let base_common = widget_style.tree.style();
        let mut style = widget_style.tree.full_key_styles();
        style.insert(WIDGET_KEY.into(), base_common);

        WidgetStyleMediator {
            palette: widget_style.palette,
            style,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "WidgetStyleMediator", into = "WidgetStyleMediator")]
pub struct WidgetStyle {
    palette: Palette,
    tree: StyleTree,
}

impl WidgetStyle {
    pub fn default_dark() -> Self {
        Self {
            palette: Palette::dark(),
            tree: StyleTree::default(),
        }
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::custom("custom", self.palette.clone().into())
    }

    pub fn palette(&self) -> &Palette {
        &self.palette
    }

    pub fn style_tree(&self) -> &StyleTree {
        &self.tree
    }

    pub fn style_tree_mut(&mut self) -> &mut StyleTree {
        &mut self.tree
    }
}
