use crate::{
    shared::{OrderedMap, OrderedSet},
    stage2::structure::{Definition, Environment, ItemId, Variable, VariableId},
};

#[derive(Debug)]
pub(super) struct DepQueryResult<'x> {
    pub(super) vars: OrderedSet<VariableId<'x>>,
    pub(super) partial_over: OrderedSet<ItemId<'x>>,
}

impl<'x> DepQueryResult<'x> {
    pub fn new() -> Self {
        Self::empty(OrderedSet::new())
    }

    pub fn empty(partial_over: OrderedSet<ItemId<'x>>) -> Self {
        Self {
            vars: Default::default(),
            partial_over,
        }
    }

    pub fn full(vars: OrderedSet<VariableId<'x>>) -> Self {
        Self {
            vars,
            partial_over: OrderedSet::new(),
        }
    }

    pub fn append(&mut self, other: Self) {
        let sv = std::mem::take(&mut self.vars);
        self.vars = sv.union(other.vars);
        let spo = std::mem::take(&mut self.partial_over);
        self.partial_over = spo.union(other.partial_over);
    }

    pub fn remove_partial(&mut self, over: ItemId<'x>) {
        self.partial_over.remove(&over);
    }
}
