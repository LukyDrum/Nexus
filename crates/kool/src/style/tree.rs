use std::{
    collections::{BTreeMap, HashMap, hash_map::Entry},
    rc::Rc,
};

use crate::style::{CommonStyle, StyleKey};

#[derive(Clone, Debug, Default)]
pub struct StyleTree {
    style: CommonStyle,
    subtrees: HashMap<StyleKey, Rc<StyleTree>>,
}

impl StyleTree {
    /// Sets the `style` for this style tree node.
    pub fn set_style(&mut self, style: CommonStyle) {
        self.style = style;
    }

    pub fn insert(&mut self, key: impl Into<StyleKey>, style: CommonStyle) {
        let key = key.into();
        if key.is_empty() {
            self.set_style(style);
            return;
        }

        let (base, rest) = key.base_and_rest();
        let rest = rest.unwrap_or_default();
        match self.subtrees.entry(base) {
            Entry::Occupied(mut occupied) => {
                Rc::get_mut(occupied.get_mut())
                    .expect("It is up to the user to make sure there are no other references.")
                    .insert(rest, style);
            }
            Entry::Vacant(vacant) => {
                let mut subtree = StyleTree::default();
                subtree.insert(rest, style);

                vacant.insert(Rc::new(subtree));
            }
        }
    }

    pub fn nest(&mut self, key: impl Into<StyleKey>, subtree: Rc<StyleTree>) {
        let key = key.into();
        if key.is_base() {
            self.subtrees.insert(key, subtree);
            return;
        }

        let (base, rest) = key.base_and_rest();
        let rest = rest.unwrap_or_default();
        match self.subtrees.entry(base) {
            Entry::Occupied(mut occupied) => {
                Rc::get_mut(occupied.get_mut())
                    .expect("It is up to the user to make sure there are no other references.")
                    .nest(rest, subtree);
            }
            Entry::Vacant(vacant) => {
                let mut middle = StyleTree::default();
                middle.nest(rest, subtree);

                vacant.insert_entry(Rc::new(middle));
            }
        }
    }

    /// Returns the style of this style tree node.
    pub fn style(&self) -> CommonStyle {
        self.style
    }

    /// Gets the style from this style tree.
    /// If key is not found, then the style of this node is returned.
    /// Non-defined properties of children styles get overriden by their parents.
    pub fn get(&self, key: impl Into<StyleKey>) -> CommonStyle {
        let key = key.into();
        if key.is_empty() {
            return self.style();
        }

        let (base, rest) = key.base_and_rest();
        let rest = rest.unwrap_or_default();
        self.subtrees
            .get(&base)
            .map(|sub_tree| sub_tree.get(rest).inherit(&self.style()))
            .unwrap_or_else(|| self.style())
    }

    /// Returns the whole subtree belonging to the `key`.
    /// Other queries can be made inside this subtree with it acting as a root.
    pub fn subtree(this: &Rc<Self>, key: impl Into<StyleKey>) -> Rc<Self> {
        let key = key.into();
        if key.is_empty() {
            return this.clone();
        }

        let (base, rest) = key.base_and_rest();
        let rest = rest.unwrap_or_default();
        this.subtrees
            .get(&base)
            .map(|base_tree| Self::subtree(base_tree, rest))
            .unwrap_or_else(|| this.clone())
    }

    pub(super) fn full_key_styles(&self) -> BTreeMap<StyleKey, CommonStyle> {
        self.subtrees
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
