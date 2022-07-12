pub mod vomit;

use std::collections::HashMap;

use crate::item::{definitions::other::DOther, Item, ItemDefinition, ItemPtr};

#[cfg(not(feature = "no_axioms"))]
pub const LANGUAGE_ITEM_NAMES: &[&str] = &[
    "true",
    "false",
    "void",
    "x",
    "y",
    "when_equal",
    "when_not_equal",
    "and",
    "trivial_t_statement",
    "invariant_truth_t_statement",
    "invariant_truth_rev_t_statement",
    "eq_ext_rev_t_statement",
    "inv_eq_t_statement",
    "refl_t_statement",
    "cases_t_statement",
    "decision_eq_t_statement",
    "decision_neq_t_statement",
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
            let id = Item::placeholder(format!("language item {}", name));
            this.language_items.insert(name, id);
        }
        this
    }

    pub fn define_language_item(&mut self, name: &str, definition: ItemPtr) -> Option<()> {
        let id = self.get_language_item(name)?;
        id.redefine(DOther::new(definition).clone_into_box());
        Some(())
    }

    #[track_caller]
    pub fn get_language_item(&self, name: &str) -> Option<&ItemPtr> {
        self.language_items.get(name)
    }

    pub fn get_true(&self) -> &ItemPtr {
        self.get_language_item("true").unwrap()
    }

    pub fn get_false(&self) -> &ItemPtr {
        self.get_language_item("false").unwrap()
    }

    pub fn get_void(&self) -> &ItemPtr {
        self.get_language_item("void").unwrap()
    }

    pub(crate) fn language_item_names(&self) -> impl Iterator<Item = &'static str> {
        LANGUAGE_ITEM_NAMES.iter().copied()
    }

    pub(crate) fn add_auto_theorem(&mut self, auto_theorem: ItemPtr) {
        self.auto_theorems.push(auto_theorem)
    }
}
