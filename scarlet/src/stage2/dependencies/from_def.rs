use itertools::Itertools;

use super::structures::DepQueryResult;
use crate::stage2::structure::{
    Definition, Environment, ImplicitlyLowered, ItemId, VarType, VariableId, VariableInfo,
};

impl<'x> Environment<'x> {
    pub(super) fn get_deps_from_def(
        &mut self,
        of: ItemId<'x>,
        after: &[VariableId<'x>],
    ) -> DepQueryResult<'x> {
        match self.get_definition(of).clone() {
            Definition::After { base, vals } => {
                let mut after_vars = DepQueryResult::new();
                for val in vals {
                    after_vars.append(self.dep_query(val, &[]));
                }
                let extra_partial_overs = after_vars.partial_over;
                let after_vars = after_vars.deps.into_iter().map(|x| x.0.var).collect_vec();

                let new_after = [after.to_owned(), after_vars].concat();
                let mut result = self.dep_query(base, &new_after[..]);
                result.partial_over = result.partial_over.union(extra_partial_overs);
                result
            }
            Definition::BuiltinOperation(_, args) => {
                let mut result = DepQueryResult::new();
                for arg in args {
                    result.append(self.dep_query(arg, &[]));
                }
                result
            }
            Definition::BuiltinValue(_) => DepQueryResult::new(),
            Definition::Match {
                base,
                conditions,
                else_value,
            } => todo!(),
            Definition::Member(_, _) => todo!(),
            Definition::Other(other) => self.dep_query(other, after),
            Definition::Struct(fields) => {
                let mut query = DepQueryResult::new();
                for field in fields {
                    query.append(self.dep_query(field.value, &[]).with_only(after));
                }
                query
            }
            Definition::UnresolvedSubstitute(..) => {
                self.resolve_substitution(of);
                self.get_deps_from_def(of, after)
            }
            Definition::ResolvedSubstitute(_, _) => todo!(),
            Definition::Variable { var, typee } => {
                let mut result = self.deps_of_var_typ(typee, after).with_only(after);
                let this = VariableInfo {
                    var_item: of,
                    var,
                    typee,
                    lifted: ImplicitlyLowered,
                };
                result.deps.insert_or_replace(this, ());
                result
            }
        }
    }

    fn deps_of_var_typ(
        &mut self,
        typee: VarType<'x>,
        after: &[VariableId<'x>],
    ) -> DepQueryResult<'x> {
        match typee {
            VarType::God | VarType::_32U | VarType::Bool => DepQueryResult::new(),
            VarType::Just(other) => self.dep_query(other, after),
            VarType::And(left, right) => {
                let mut result = self.dep_query(left, after);
                result.append(self.dep_query(right, after));
                result
            }
        }
    }
}
