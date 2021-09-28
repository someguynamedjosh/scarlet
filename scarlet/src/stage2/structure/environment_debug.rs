use std::fmt::{self, Debug, Formatter};

use super::{Environment, ItemDefinition};
use crate::{shared::ItemId, stage2::structure::UnresolvedItem, util::indented};

fn fmt_item(f: &mut Formatter, item: &UnresolvedItem) -> fmt::Result {
    if f.alternate() {
        let text = format!("{:#?}", item);
        write!(f, "{},", indented(&text[..]))
    } else {
        write!(f, "{:?}", item)
    }
}

fn fmt_item_prefixes(f: &mut Formatter, item: &ItemDefinition) -> fmt::Result {
    if let Some(scope) = item.info_requested {
        write!(f, "info{{in {:?}}}", scope)?;
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

fn maybe_fmt_environment_item(
    f: &mut Formatter,
    index: usize,
    item: &ItemDefinition,
) -> fmt::Result {
    if f.alternate() {
        write!(f, "\n\n    ")?;
    }
    fmt_item_prefixes(f, item)?;
    write!(f, "{:?} is ", ItemId(index))?;
    match &item.definition {
        Some(item) => fmt_item(f, item),
        None => write!(f, "None,"),
    }
}

impl Debug for Environment {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Environment [")?;
        for (index, item) in self.items.iter().enumerate() {
            maybe_fmt_environment_item(f, index, item)?;
        }
        if f.alternate() {
            writeln!(f)?;
        }
        write!(f, "]")
    }
}
