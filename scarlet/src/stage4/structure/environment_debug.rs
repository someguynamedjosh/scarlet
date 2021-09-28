use std::fmt::{self, Debug, Formatter};

use super::{Environment, ItemDefinition};
use crate::{shared::ItemId, util::indented};

fn fmt_item_prefixes(f: &mut Formatter, item: &ItemDefinition) -> fmt::Result {
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

fn fmt_item_definition(f: &mut Formatter, index: usize, item: &ItemDefinition) -> fmt::Result {
    write!(f, "{:?} is ", ItemId(index))?;
    if f.alternate() {
        let text = format!("{:#?}", item.definition);
        write!(f, "{}", indented(&text[..]))
    } else {
        write!(f, "{:?}", item.definition)
    }
}

fn fmt_type_annotation(f: &mut Formatter, item: &ItemDefinition) -> fmt::Result {
    match item.cached_type {
        Some(typee) => write!(f, "\n    :{:?}", typee),
        None => write!(f, "\n    :?"),
    }
}

fn fmt_item(f: &mut Formatter, index: usize, item: &ItemDefinition) -> fmt::Result {
    if f.alternate() {
        write!(f, "\n\n    ")?;
    }
    fmt_item_prefixes(f, item)?;
    fmt_item_definition(f, index, item)?;
    fmt_type_annotation(f, item)
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
