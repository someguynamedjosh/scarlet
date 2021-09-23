use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};

use crate::{
    shared::{Item, ItemId},
    stage3::structure::{self as stage3},
    util,
};

#[derive(Clone, PartialEq)]
pub struct TypedItem {
    pub base: Item,
    pub typee: Option<ItemId>,
}

#[derive(Clone, PartialEq)]
pub struct Environment {
    pub modules: Vec<ItemId>,
    pub items: Vec<TypedItem>,
    pub item_reverse_lookup: HashMap<Item, ItemId>,
}

fn reverse_lookups(from: &stage3::Environment) -> HashMap<Item, ItemId> {
    from.items
        .iter()
        .enumerate()
        .map(|(index, item)| (item.clone(), ItemId(index)))
        .collect()
}

fn items(items: Vec<Item>) -> Vec<TypedItem> {
    items
        .into_iter()
        .map(|i| TypedItem {
            base: i,
            typee: None,
        })
        .collect()
}

impl Environment {
    pub fn _new_empty() -> Self {
        Self::new(stage3::Environment::new())
    }

    pub fn new(from: stage3::Environment) -> Self {
        Self {
            item_reverse_lookup: reverse_lookups(&from),
            modules: from.modules,
            items: items(from.items),
        }
    }
}

fn fmt_item(f: &mut Formatter, index: usize, item: &TypedItem) -> fmt::Result {
    write!(f, "{:?} is ", ItemId(index))?;
    if f.alternate() {
        let text = format!("{:#?}", item.base);
        write!(f, "{}\n    ", util::indented(&text))
    } else {
        write!(f, "{:?} ", item.base)
    }
}

fn fmt_type_annotation(f: &mut Formatter, item: &TypedItem) -> fmt::Result {
    write!(f, "type_is{{ ")?;
    match &item.typee {
        Some(item) => write!(f, "{:?}", item)?,
        None => write!(f, "?")?,
    }
    write!(f, " }}")
}

fn fmt_typed_item(f: &mut Formatter, index: usize, item: &TypedItem) -> fmt::Result {
    if f.alternate() {
        write!(f, "\n\n    ")?;
    }
    fmt_item(f, index, item)?;
    fmt_type_annotation(f, item)
}

impl Debug for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Environment[")?;
        for (index, item) in self.items.iter().enumerate() {
            fmt_typed_item(f, index, item)?;
        }
        if f.alternate() {
            writeln!(f)?;
        }
        write!(f, "]")
    }
}
