pub mod from;
pub mod recursion;
pub mod resolve;
pub mod vomit;

use std::collections::HashMap;

use crate::{item::ItemPtr, scope::SRoot};

#[cfg(not(feature = "no_axioms"))]
pub const LANGUAGE_ITEM_NAMES: &[&str] = &[
    "true",
    "false",
    "void",
    "x",
    "and",
    "t_trivial_statement",
    "t_invariant_truth_statement",
    "t_invariant_truth_rev_statement",
    "t_eq_ext_rev_statement",
    "t_inv_eq_statement",
    "t_refl_statement",
    "t_decision_eq_statement",
    "t_decision_neq_statement",
];

#[cfg(feature = "no_axioms")]
pub const LANGUAGE_ITEM_NAMES: &[&str] = &["true", "false", "void", "x", "and"];

#[derive(Debug)]
pub struct Environment {
    language_items: HashMap<&'static str, ItemPtr>,
    pub(super) auto_theorems: Vec<ItemPtr>,
}

impl Environment {
    pub fn new() -> Self {
        let mut this = Self {
            language_items: HashMap::new(),
            auto_theorems: Vec::new(),
        };
        for &name in LANGUAGE_ITEM_NAMES {
            let id = this.push_placeholder(Box::new(SRoot));
            this.language_items.insert(name, id);
        }
        this
    }

    pub fn define_language_item(&mut self, name: &str, definition: ItemPtr) {
        let id = self.get_language_item(name);
        self.items[id].definition = definition.into();
    }

    #[track_caller]
    pub fn get_language_item(&self, name: &str) -> ItemPtr {
        *self
            .language_items
            .get(name)
            .expect(&format!("nice error, no language item named {}", name))
    }

    pub(crate) fn language_item_names(&self) -> impl Iterator<Item = &'static str> {
        LANGUAGE_ITEM_NAMES.iter().copied()
    }
}
