use itertools::Itertools;

use super::structures::DepQueryResult;
use crate::stage2::structure::{
    Definition, Environment, ImplicitlyLowered, ItemId, VarType, VariableId, VariableInfo,
};

impl<'x> Environment<'x> {
    pub(super) fn get_deps_from_def(&mut self, of: ItemId<'x>) -> DepQueryResult<'x> {
        match self.get_definition(of).clone() {
            Definition::BuiltinOperation(_, args) => {
                let mut result = DepQueryResult::new();
                for arg in args {
                    result.append(self.dep_query(arg));
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
            Definition::Other(other) => self.dep_query(other),
            Definition::Struct(fields) => {
                let mut query = DepQueryResult::new();
                for field in fields {
                    query.append(self.dep_query(field.value));
                }
                query
            }
            Definition::UnresolvedSubstitute(..) => {
                self.resolve_substitution(of);
                self.get_deps_from_def(of)
            }
            Definition::ResolvedSubstitute(_, _) => todo!(),
            Definition::Variable { var, typee } => {
                let mut result = self.deps_of_var_typ(typee);
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

    fn deps_of_var_typ(&mut self, typee: VarType<'x>) -> DepQueryResult<'x> {
        match typee {
            VarType::God | VarType::_32U | VarType::Bool => DepQueryResult::new(),
            VarType::Just(other) => self.dep_query(other),
            VarType::And(left, right) => {
                let mut result = self.dep_query(left);
                result.append(self.dep_query(right));
                result
            }
        }
    }
}
