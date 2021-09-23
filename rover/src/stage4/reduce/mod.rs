use std::collections::HashMap;

use crate::{
    shared::{
        Definitions, IntegerMathOperation, Item, ItemId, PrimitiveOperation, PrimitiveValue,
        Replacements,
    },
    stage4::structure::Environment,
};

mod reduce;
mod reduce_basics;
mod reduce_inductive_value;
mod reduce_is_same_variant;
mod reduce_pick;
mod reduce_primitive_operation;
mod reduce_replacing;
mod replacements;

pub type Reps = HashMap<ItemId, ItemId>;

#[derive(Clone, Copy)]
pub struct ReduceOptions<'a> {
    item: ItemId,
    reps: &'a Reps,
    reduce_defs: bool,
}

impl<'a> ReduceOptions<'a> {
    fn with_item(mut self, item: ItemId) -> Self {
        self.item = item;
        self
    }
}

pub fn reduce(env: &mut Environment) {
    let mut replacements = HashMap::new();
    let mut id = ItemId(0);
    while id.0 < env.items.len() {
        env.compute_type(id).unwrap();
        let opts = ReduceOptions {
            item: id,
            reduce_defs: false,
            reps: &HashMap::new(),
        };
        let new = env.reduce(opts);
        if new != id {
            replacements.insert(id, new);
        }
        id.0 += 1
    }
    let mut id = ItemId(0);
    while id.0 < env.items.len() {
        env.apply_replacements(id, &replacements);
        id.0 += 1
    }
}

impl Environment {
    /// Returns true if the provided type does not indicate that corresponding
    /// values depend on the values of other variables.
    fn type_is_not_from(&self, typee: ItemId) -> bool {
        match &self.items[typee.0].base {
            Item::Defining { base, .. } => self.type_is_not_from(*base),
            Item::FromType { base, vars, .. } => vars.is_empty() && self.type_is_not_from(*base),
            Item::Replacing { base, .. } => self.type_is_not_from(*base),
            _ => true,
        }
    }
}
