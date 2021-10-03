use super::{Definitions, Item, NamespaceId, ReplacementsId};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Namespace {
    Defining {
        base: NamespaceId,
        definitions: Definitions,
    },
    Empty,
    Replacing {
        base: NamespaceId,
        replacements: Vec<ReplacementsId>,
    },
    Root(Item),
}
