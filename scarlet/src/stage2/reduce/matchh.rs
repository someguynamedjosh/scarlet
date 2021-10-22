use crate::{
    shared::{OrderedMap, OrderedSet},
    stage2::structure::{
        BuiltinOperation, BuiltinValue, Condition, Definition, Environment, ItemId,
        StructField, Substitution, VariableId,
    },
};

impl<'x> Environment<'x> {
    pub(super) fn matches(&mut self, pattern: ItemId<'x>, value: ItemId<'x>) -> Option<bool> {
        match self.items[pattern].definition.as_ref().unwrap() {
            Definition::BuiltinOperation(_, _) => todo!(),
            Definition::BuiltinValue(pv) => {
                match self.items[value].definition.as_ref().unwrap() {
                    Definition::BuiltinValue(vv) => Some(pv == vv),
                    Definition::Struct(..) => Some(false),
                    _ => None,
                }
            }
            Definition::Match { .. } => None,
            Definition::Member(_, _) => todo!(),
            Definition::Other(..) => unreachable!(),
            Definition::Struct(_) => todo!(),
            Definition::Substitute(..) => None,
            Definition::Variable(var) => {
                let var_pattern = self.vars[*var].pattern;
                self.matches(var_pattern, value)
            }
        }
    }
}
