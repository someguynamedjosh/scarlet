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
