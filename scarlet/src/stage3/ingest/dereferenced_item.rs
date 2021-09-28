use crate::shared::{ItemId, Replacements};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DereferencedItem {
    Stage2Item(ItemId),
    Replacing {
        base: Box<DereferencedItem>,
        replacements: Replacements,
    },
}

impl DereferencedItem {
    pub fn id(&self) -> ItemId {
        match self {
            Self::Stage2Item(id) => *id,
            Self::Replacing { base, .. } => base.id(),
        }
    }

    pub fn with_base(&self, new_base: DereferencedItem) -> Self {
        match self {
            Self::Stage2Item(..) => new_base,
            Self::Replacing { base, replacements } => Self::Replacing {
                base: Box::new(base.with_base(new_base)),
                replacements: replacements.clone(),
            },
        }
    }
}
