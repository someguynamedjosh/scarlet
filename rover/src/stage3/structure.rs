use std::fmt::{self, Debug, Formatter};

use crate::{
    shared::{Item, ItemId},
    util::indented,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Environment {
    pub modules: Vec<ItemId>,
    pub(crate) items: Vec<Item>,
}

fn fmt_item(f: &mut Formatter, index: usize, item: &Item) -> fmt::Result {
    if f.alternate() {
        write!(f, "\n\n    ")?;
    }
    write!(f, "{:?} is ", ItemId(index))?;
    if f.alternate() {
        let text = format!("{:#?}", item);
        write!(f, "{},", indented(&text[..]))
    } else {
        write!(f, "{:?}", item)
    }
}

impl Debug for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Environment [")?;
        for (index, item) in self.items.iter().enumerate() {
            fmt_item(f, index, item)?;
        }
        if f.alternate() {
            writeln!(f)?;
        }
        write!(f, "]")
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn _mark_as_module(&mut self, item: ItemId) {
        self.modules.push(item)
    }

    pub fn insert(&mut self, definition: Item) -> ItemId {
        let id = ItemId(self.items.len());
        self.items.push(definition);
        id
    }

    pub fn _definition_of(&self, item: ItemId) -> &Item {
        assert!(item.0 < self.items.len());
        &self.items[item.0]
    }
}
