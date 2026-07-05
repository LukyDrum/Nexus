use std::collections::{BTreeMap, HashMap, hash_map::Entry};

use crate::style::{CommonStyle, StyleKey};

#[derive(Clone, Debug, Default)]
pub struct StyleTree {
    style: CommonStyle,
    sub_trees: HashMap<StyleKey, Box<StyleTree>>,
}

impl StyleTree {
    pub fn set_style(&mut self, style: CommonStyle) {
        self.style = style;
    }

    pub fn insert(&mut self, key: StyleKey, style: CommonStyle) {
        if key.is_empty() {
            self.set_style(style);
            return;
        }

        let (base, rest) = key.base_and_rest();
        let rest = rest.unwrap_or_default();
        match self.sub_trees.entry(base) {
            Entry::Occupied(mut occupied) => {
                occupied.get_mut().insert(rest, style);
            }
            Entry::Vacant(vacant) => {
                let sub_tree = vacant.insert(Box::new(StyleTree::default()));
                sub_tree.insert(rest, style);
            }
        }
    }

    pub fn style(&self) -> CommonStyle {
        self.style
    }

    pub fn get(&self, key: impl Into<StyleKey>) -> CommonStyle {
        let key = key.into();
        if key.is_empty() {
            return self.style();
        }

        let (base, rest) = key.base_and_rest();
        let rest = rest.unwrap_or_default();
        self.sub_trees
            .get(&base)
            .map(|sub_tree| sub_tree.get(rest))
            .unwrap_or_else(|| self.style())
    }

    pub fn sub_tree(&self, key: impl Into<StyleKey>) -> &StyleTree {
        let key = key.into();
        if key.is_empty() {
            return self;
        }

        let (base, rest) = key.base_and_rest();
        let rest = rest.unwrap_or_default();
        self.sub_trees
            .get(&base)
            .map(|sub_tree| sub_tree.sub_tree(rest))
            .unwrap_or(self)
    }

    pub(crate) fn full_key_styles(&self) -> BTreeMap<StyleKey, CommonStyle> {
        self.sub_trees
            .iter()
            .flat_map(|(key, sub_tree)| {
                let sub_styles = sub_tree.full_key_styles();
                sub_styles
                    .into_iter()
                    .map(|(sub_key, style)| (key.clone().join(&sub_key), style))
            })
            .collect()
    }
}
