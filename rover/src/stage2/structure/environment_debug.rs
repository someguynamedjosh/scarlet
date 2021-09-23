use std::fmt::{self, Debug, Formatter};

use super::Environment;
use crate::{shared::ItemId, stage2::structure::UnresolvedItem, util::indented};

fn fmt_environment_item(f: &mut Formatter, _index: usize, item: &UnresolvedItem) -> fmt::Result {
    if f.alternate() {
        let text = format!("{:#?}", item);
        write!(f, "{},", indented(&text[..]))
    } else {
        write!(f, "{:?}", item)
    }
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

fn maybe_fmt_environment_item(
    f: &mut Formatter,
    env: &Environment,
    index: usize,
    item: &Option<UnresolvedItem>,
) -> fmt::Result {
    if f.alternate() {
        write!(f, "\n\n    ")?;
    }
    fmt_item_prefixes(f, env, index)?;
    write!(f, "{:?} is ", ItemId(index))?;
    match item {
        Some(item) => fmt_environment_item(f, index, item),
        None => write!(f, "None,"),
    }
}

impl Debug for Environment {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Environment [")?;
        for (index, item) in self.items.iter().enumerate() {
            maybe_fmt_environment_item(f, self, index, item)?;
        }
        if f.alternate() {
            writeln!(f)?;
        }
        write!(f, "]")
    }
}
