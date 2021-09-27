use std::collections::HashMap;

use crate::{
    shared::{Item, ItemId},
    stage4::{ingest::var_list::VarList, structure::Environment},
};

mod reduce;
mod reduce_basics;
mod reduce_builtin_operation;
mod reduce_pick;
mod reduce_replacing;
mod replacements;

pub type Reps = HashMap<ItemId, ItemId>;

#[derive(Clone, Copy)]
pub struct ReduceOptions<'a> {
    item: ItemId,
    defined_in: Option<ItemId>,
    reps: &'a Reps,
    reduce_defs: bool,
}

impl<'a> ReduceOptions<'a> {
    fn with_item(mut self, item: ItemId) -> Self {
        self.item = item;
        self
    }
}

fn reduce_item(env: &mut Environment, id: ItemId) -> ItemId {
    env.compute_type(id, vec![]).unwrap();
    let defined_in = env.items[id.0].defined_in;
    let opts = ReduceOptions {
        item: id,
        defined_in,
        reduce_defs: false,
        reps: &HashMap::new(),
    };
    env.reduce(opts)
}

fn apply_replacements(env: &mut Environment, reps: Reps) {
    let mut id = ItemId(0);
    while id.0 < env.items.len() {
        env.apply_replacements(id, &reps);
        id.0 += 1
    }
}

pub fn reduce(env: &mut Environment) {
    let mut replacements = HashMap::new();
    let mut id = ItemId(0);
    while id.0 < env.items.len() {
        let new = reduce_item(env, id);
        if new != id {
            replacements.insert(id, new);
        }
        id.0 += 1
    }
    apply_replacements(env, replacements)
}

impl Environment {
    /// Returns true if the provided type does not indicate that corresponding
    /// values depend on the values of other variables.
    fn type_is_not_from(&self, typee: ItemId) -> bool {
        match &self.items[typee.0].definition {
            Item::Defining { base, .. } => self.type_is_not_from(*base),
            Item::FromType { base, values, .. } => {
                values.is_empty() && self.type_is_not_from(*base)
            }
            Item::Replacing { base, .. } => self.type_is_not_from(*base),
            _ => true,
        }
    }

    fn type_depends_on_nothing_except(&self, typee: ItemId, allowed_deps: &VarList) -> bool {
        match &self.items[typee.0].definition {
            Item::Defining { base, .. } => self.type_depends_on_nothing_except(*base, allowed_deps),
            Item::FromType { base, values, .. } => {
                for value in values {
                    if !allowed_deps.as_slice().contains(value) {
                        return false;
                    }
                }
                self.type_depends_on_nothing_except(*base, allowed_deps)
            }
            Item::Replacing { base, .. } => self.type_depends_on_nothing_except(*base, allowed_deps),
            _ => true,
        }
    }
}
