use super::{Definitions, Item, NamespaceId, Replacements};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Namespace {
    Defining {
        base: NamespaceId,
        definitions: Definitions,
        parent: NamespaceId,
    },
    Empty,
    Identifier {
        name: String,
        in_namespace: NamespaceId,
    },
    Member {
        base: NamespaceId,
        name: String,
    },
    Replacing {
        base: NamespaceId,
        replacements: Replacements,
    },
    Root(Item),
}