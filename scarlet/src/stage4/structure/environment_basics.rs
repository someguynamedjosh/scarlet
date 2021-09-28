use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};

use crate::{
    shared::{Item, ItemId},
    stage3::structure::{self as stage3, ItemDefinition},
    stage4::ingest::var_list::VarList,
    util,
};

#[derive(Clone, PartialEq)]
pub struct TypedItem {
    /// True when the programmer has requested a diagnostic showing information
    /// about this definition. Contains the scope from which the information was
    /// requested.
    pub info_requested: Option<ItemId>,
    /// True if this item is a place where other items are defined.
    pub is_scope: bool,
    pub definition: Item,
    pub defined_in: Option<ItemId>,
    pub typee: Option<ItemId>,
    pub reduction_blockers: VarList,
}

#[derive(Clone, PartialEq)]
pub struct Environment {
    pub items: Vec<TypedItem>,
    pub item_reverse_lookup: HashMap<Item, ItemId>,
}

fn reverse_lookups(from: &stage3::Environment) -> HashMap<Item, ItemId> {
    from.items
        .iter()
        .enumerate()
        .map(|(index, item)| (item.definition.clone(), ItemId(index)))
        .collect()
}

fn items(items: Vec<ItemDefinition>) -> Vec<TypedItem> {
    items
        .into_iter()
        .map(|i| TypedItem {
            info_requested: i.info_requested,
            is_scope: i.is_scope,
            definition: i.definition,
            defined_in: i.defined_in,
            typee: None,
            reduction_blockers: VarList::new(),
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
            items: items(from.items),
        }
    }

    pub fn deref_replacing_and_defining(&self, val: ItemId) -> ItemId {
        match &self.items[val.0].definition {
            Item::Defining { base, .. } | Item::Replacing { base, .. } => {
                self.deref_replacing_and_defining(*base)
            }
            _ => val,
        }
    }

    pub fn get(&self, id: ItemId) -> &TypedItem {
        assert!(id.0 < self.items.len());
        &self.items[id.0]
    }
}

fn fmt_item_prefixes(f: &mut Formatter, item: &TypedItem) -> fmt::Result {
    if let Some(scope) = item.info_requested {
        write!(f, "info{{in {:?}}} ", scope)?;
    }
    if item.is_scope {
        write!(f, "scope ")?;
    }
    if let Some(scope) = item.defined_in {
        write!(f, "in {:?}, ", scope)
    } else {
        write!(f, "root, ")
    }
}

fn fmt_item(f: &mut Formatter, index: usize, item: &TypedItem) -> fmt::Result {
    write!(f, "{:?} is ", ItemId(index))?;
    if f.alternate() {
        let text = format!("{:#?}", item.definition);
        write!(f, "{}\n    ", util::indented(&text))
    } else {
        write!(f, "{:?} ", item.definition)
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
    fmt_item_prefixes(f, item)?;
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
