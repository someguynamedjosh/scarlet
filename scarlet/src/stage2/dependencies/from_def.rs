mod others;
mod variable;

use super::structures::DepQueryResult;
use crate::stage2::structure::{Definition, Environment, ItemId};

impl<'x> Environment<'x> {
    pub(super) fn get_deps_from_def(
        &mut self,
        of: ItemId<'x>,
        num_struct_unwraps: u32,
    ) -> DepQueryResult<'x> {
        match self.get_definition(of).clone() {
            Definition::BuiltinOperation(_, args) => {
                self.deps_of_builtin_op(args, num_struct_unwraps)
            }
            Definition::BuiltinValue(..) => DepQueryResult::new(),
            Definition::Match {
                base,
                conditions,
                else_value,
            } => self.deps_of_match(base, num_struct_unwraps, conditions, else_value),
            Definition::Member(base, _) => self.dep_query(base, num_struct_unwraps + 1),
            Definition::Resolvable { .. } => {
                let of = self.resolve(of);
                self.get_deps_from_def(of, num_struct_unwraps)
            }
            Definition::SetEager { base, vals, eager } => {
                self.deps_of_set_eager(vals, num_struct_unwraps, base, of, eager)
            }
            Definition::Struct(fields) => {
                let mut query = DepQueryResult::new();
                for field in fields {
                    if num_struct_unwraps == 0 {
                        query.append(self.dep_query(field.value, 0).discarding_shy());
                    } else {
                        query.append(self.dep_query(field.value, num_struct_unwraps - 1));
                    }
                }
                query
            }
            Definition::Substitute(base, subs) => {
                self.deps_of_resolved_substitution(base, num_struct_unwraps, subs)
            }
            Definition::Variable { var, typee } => {
                self.deps_of_variable(typee, num_struct_unwraps, of, var)
            }
        }
    }
}
