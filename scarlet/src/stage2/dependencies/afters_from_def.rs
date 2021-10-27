use super::structures::DepQueryResult;
use crate::{
    shared::OrderedSet,
    stage2::structure::{BuiltinOperation, Definition, Environment, ItemId, VariableId},
};

impl<'x> Environment<'x> {
    pub(super) fn get_afters_from_def(&mut self, of: ItemId<'x>) -> DepQueryResult<'x> {
        match self.items[of].definition.clone().unwrap() {
            Definition::After { base, vals } => {
                let mut result = DepQueryResult::new();
                for val in vals {
                    result.append(self.dep_query(val));
                }
                result.append(self.after_query(base));
                result
            }
            Definition::BuiltinOperation(_, args) => {
                let mut result = DepQueryResult::new();
                for arg in args {
                    result.append(self.after_query(arg));
                }
                result
            }
            Definition::BuiltinPattern(..) => DepQueryResult::new(),
            Definition::BuiltinValue(..) => DepQueryResult::new(),
            Definition::Match {
                base,
                conditions,
                else_value,
            } => {
                let mut result = self.after_query(base);
                result.append(self.after_query(else_value));
                for cond in conditions {
                    result.append(self.after_query(cond.pattern));
                    result.append(self.after_query(cond.value));
                }
                result
            }
            Definition::Member(base, _) => self.after_query(base),
            Definition::Other(other) => self.after_query(other),
            Definition::Struct(fields) => {
                let mut result = DepQueryResult::new();
                for field in fields {
                    result.append(self.after_query(field.value));
                }
                result
            }
            Definition::Substitute(..) => DepQueryResult::new(),
            Definition::Variable(var) => self.after_query(self.vars[var].pattern),
        }
    }
}
