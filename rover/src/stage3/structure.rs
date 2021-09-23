use std::fmt::{self, Debug, Formatter};

use crate::{
    shared::{Item, ItemId},
    util::indented,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Environment {
    pub infos: Vec<ItemId>,
    pub modules: Vec<ItemId>,
    pub(crate) items: Vec<Item>,
}

fn fmt_item_prefixes(f: &mut Formatter, env: &Environment, index: usize) -> fmt::Result {
    let id = ItemId(index);
    if env.infos.contains(&id) {
        write!(f, "info ")?;
    }
    if env.modules.contains(&id) {
        write!(f, "module ")?;
    }
    Ok(())
}

fn fmt_item(f: &mut Formatter, env: &Environment, index: usize, item: &Item) -> fmt::Result {
    if f.alternate() {
        write!(f, "\n\n    ")?;
    }
    fmt_item_prefixes(f, env, index)?;
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
            if self.infos.contains(&ItemId(index)) {
                write!(f, "info ")?;
            }
            fmt_item(f, self, index, item)?;
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
            infos: Vec::new(),
            modules: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn mark_info(&mut self, item: ItemId) {
        self.infos.push(item)
    }

    pub fn mark_as_module(&mut self, item: ItemId) {
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
