use std::collections::HashMap;

use crate::{
    shared::{Item, ItemId},
    stage2::{self, structure::UnresolvedItem},
    stage3::structure::ItemDefinition,
};

pub(super) struct Context {
    pub src: stage2::structure::Environment,
    pub stage2_to_stage3: HashMap<ItemId, ItemId>,
    pub stage3_items: Vec<(ItemId, ItemDefinition)>,
    /// Item for which info is requested, scope the info was requested from.
    pub info_requests: Vec<(ItemId, ItemId)>,
    pub next_stage3_id: ItemId,
}

impl Context {
    /// Returns a stage 2 id
    pub fn get_member(&self, from: ItemId, name: &str) -> Result<ItemId, String> {
        let def = self.src.get(from);
        let item = def.definition.as_ref().expect("ICE: Undefined Item");
        if let UnresolvedItem::Just(Item::Defining { base, definitions }) = item {
            if let Ok(member) = self.get_member(*base, name) {
                return Ok(member);
            } else {
                for def in definitions {
                    if def.0 == name {
                        return Ok(def.1);
                    }
                }
            }
        } else if let UnresolvedItem::Just(Item::Replacing { base, .. }) = item {
            return self.get_member(*base, name)
        } else if let UnresolvedItem::Item(base) = item {
            return self.get_member(*base, name)
        } else if let UnresolvedItem::Member{ base: other_base, name: other_name } = item {
            return self.get_member(self.get_member(*other_base, other_name)?, name)
        }
        Err(format!("{:?} has no member named {}", from, name))
    }

    pub fn reserve_new_item(&mut self, s2_id: ItemId) -> ItemId {
        let s3_id = self.next_stage3_id;
        self.next_stage3_id.0 += 1;
        self.stage2_to_stage3.insert(s2_id, s3_id);
        s3_id
    }
}
