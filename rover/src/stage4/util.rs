use crate::{
    stage2::structure::{ItemId, PrimitiveType},
    stage3::structure::Item,
    stage4::structure::Environment,
};

impl Environment {
    pub(super) fn existing_item(&self, def: &Item) -> Option<ItemId> {
        for (index, item) in self.items.iter().enumerate() {
            if &item.base == def {
                return Some(ItemId(index));
            }
        }
        None
    }

    pub(super) fn god_type(&self) -> ItemId { self.existing_item(&Item::GodType).unwrap() }

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

    /// Returns the type of the variable given by the id, assuming the id points
    /// to a variable.
    pub(super) fn get_var_type(&self, var: ItemId) -> ItemId {
        match &self.items[var.0].base {
            Item::Variable { typee, .. } => *typee,
            _ => panic!("{:?} is not a variable", var),
        }
    }
}
