use crate::{
    shared::{Item, ItemId, PrimitiveType, Replacements},
    stage4::structure::Environment,
};

impl Environment {
    pub(super) fn existing_item(&self, def: &Item) -> Option<ItemId> {
        for (index, item) in self.items.iter().enumerate() {
            if &item.definition == def {
                return Some(ItemId(index));
            }
        }
        None
    }

    pub(super) fn god_type(&self) -> ItemId {
        self.existing_item(&Item::GodType).unwrap()
    }

    pub(super) fn bool_type(&self) -> ItemId {
        self.existing_item(&Item::PrimitiveType(PrimitiveType::Bool))
            .unwrap()
    }

    pub(super) fn i32_type(&self) -> ItemId {
        self.existing_item(&Item::PrimitiveType(PrimitiveType::I32))
            .unwrap()
    }

    /// Returns true if the two items are defined as the same. This check does
    /// not always return true when this is the case, due to Godel-related math
    /// gremlins.
    pub(super) fn are_def_equal(&self, left: ItemId, right: ItemId) -> bool {
        left == right
    }

    pub(super) fn are_def_equal_after_replacements(
        &self,
        left: ItemId,
        right: ItemId,
        replacements: &Replacements,
    ) -> bool {
        let left = Self::full_replace(left, replacements);
        let right = Self::full_replace(right, replacements);
        self.are_def_equal(left, right)
    }

    fn full_replace(start: ItemId, replacements: &Replacements) -> ItemId {
        let mut result = start;
        while let Some((_, with)) = replacements.iter().filter(|(t, _)| *t == result).next() {
            result = *with;
        }
        result
    }

    /// Returns the type of the variable given by the id, assuming the id points
    /// to a variable.
    pub(super) fn get_var_type(&self, var: ItemId) -> ItemId {
        match &self.items[var.0].definition {
            Item::Variable { typee, .. } => *typee,
            _ => panic!("{:?} is not a variable", var),
        }
    }

    /// Returns the type of the given item, with any from clauses discarded.
    pub(super) fn item_base_type(&self, of: ItemId) -> ItemId {
        self.after_from(self.items[of.0].typee.unwrap())
    }

    /// If the given item is a From type, returns the base type aka the type
    /// that will be produced after the variables are replaced.
    pub(super) fn after_from(&self, typee: ItemId) -> ItemId {
        match &self.items[typee.0].definition {
            Item::Defining { base, .. } => self.after_from(*base),
            Item::FromType { base, .. } => self.after_from(*base),
            Item::GodType
            | Item::InductiveValue { .. }
            | Item::IsSameVariant { .. }
            | Item::Pick { .. }
            | Item::PrimitiveOperation(..)
            | Item::PrimitiveType(..)
            | Item::PrimitiveValue(..) => typee,
            Item::Replacing { .. } => todo!(),
            Item::TypeIs { base, .. } => self.after_from(*base),
            Item::Variable { .. } => typee,
        }
    }
}
