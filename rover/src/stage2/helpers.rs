use crate::{
    stage1::structure::expression::Expression,
    stage2::structure::{Definitions, Environment, ItemId},
};

pub(super) struct UnprocessedItem {
    pub id: ItemId,
    pub public: bool,
    pub name: String,
    pub def: Expression,
}

pub(super) fn expect_ident_expr(expr: Expression) -> Result<String, String> {
    if expr.others.len() > 0 {
        todo!("nice error")
    } else {
        expr.root.expect_ident().map(String::from)
    }
}

pub(super) fn resolve_ident(ident: &str, parents: &[&Definitions]) -> Result<ItemId, String> {
    // Search the closest parents first.
    for parent in parents.iter().rev() {
        for (name, val) in *parent {
            if name == ident {
                return Ok(*val);
            }
        }
    }
    Err(format!(
        "Could not find an item named {} in the current scope or its parents.",
        ident
    ))
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum Context {
    Plain,
    Type(ItemId),
}

pub(super) fn get_or_put_into(into: &mut Option<ItemId>, env: &mut Environment) -> ItemId {
    match into {
        Some(id) => *id,
        None => {
            let id = env.next_id();
            *into = Some(id);
            id
        }
    }
}
