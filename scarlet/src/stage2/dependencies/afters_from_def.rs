use super::structures::DepQueryResult;
use crate::{
    shared::OrderedSet,
    stage2::structure::{BuiltinOperation, Definition, Environment, ItemId, VariableId},
};

impl<'x> Environment<'x> {
    pub(super) fn get_afters_from_def(&mut self, of: ItemId<'x>) -> DepQueryResult<'x> {
        match self.items[of].definition.clone().unwrap() {
            Definition::BuiltinOperation(_, _) => todo!(),
            Definition::BuiltinValue(_) => todo!(),
            Definition::Match {
                base,
                conditions,
                else_value,
            } => todo!(),
            Definition::Member(_, _) => todo!(),
            Definition::Other(_) => todo!(),
            Definition::Struct(_) => todo!(),
            Definition::Substitute(_, _) => todo!(),
            Definition::Variable(_) => todo!(),
        }
    }
}
