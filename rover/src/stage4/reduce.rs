use std::collections::HashMap;

use crate::{stage2::structure::ItemId, stage3::structure::Item, stage4::structure::Environment};

pub fn reduce(env: &mut Environment) {

}

impl Environment {
    fn reduce(&mut self, item: ItemId, reps: HashMap<ItemId, ItemId>) -> ItemId {
        match &self.items[item.0].base {
            Item::Defining { .. } => todo!(),
            Item::FromType { .. } => todo!(),
            Item::GodType => item,
            Item::InductiveType(..) => todo!(),
            _ => todo!()
        }
    }
}
