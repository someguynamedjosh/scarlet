use std::fmt::{self, Debug, Formatter};

use crate::{
    shared::ItemId,
    stage3::structure::{Environment, ItemDefinition},
    util::indented,
};

fn fmt_item_prefixes(f: &mut Formatter, item: &ItemDefinition) -> fmt::Result {
    if item.info_requested_in.len() > 0 {
        write!(f, "info{{ ")?;
        for scope in &item.info_requested_in {
            write!(f, "{:?} ", scope)?;
        }
        write!(f, "}} ")?;
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

fn fmt_item(f: &mut Formatter, index: usize, item: &ItemDefinition) -> fmt::Result {
    if f.alternate() {
        write!(f, "\n\n    ")?;
    }
    fmt_item_prefixes(f, item)?;
    write!(f, "{:?} is ", ItemId(index))?;
    if f.alternate() {
        let text = format!("{:#?}", item.definition);
        write!(f, "{},", indented(&text[..]))
    } else {
        write!(f, "{:?}", item.definition)
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
