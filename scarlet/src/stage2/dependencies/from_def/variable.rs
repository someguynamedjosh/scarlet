use crate::stage2::{
    dependencies::structures::DepQueryResult,
    structure::{Environment, ItemId, VarType, VariableId, VariableInfo},
};

impl<'x> Environment<'x> {
    pub(super) fn deps_of_variable(
        &mut self,
        typee: VarType<'x>,
        num_struct_unwraps: u32,
        of: ItemId<'x>,
        var: VariableId<'x>,
    ) -> DepQueryResult<'x> {
        let mut result = self
            .deps_of_var_typ(typee, num_struct_unwraps)
            .discarding_shy();
        let this = VariableInfo {
            var_item: of,
            var,
            typee,
            eager: false,
        };
        result.deps.insert_or_replace(this, ());
        result
    }

    fn deps_of_var_typ(
        &mut self,
        typee: VarType<'x>,
        num_struct_unwraps: u32,
    ) -> DepQueryResult<'x> {
        match typee {
            VarType::God | VarType::_32U | VarType::Bool => DepQueryResult::new(),
            VarType::Just(other) => self.dep_query(other, num_struct_unwraps),
            VarType::And(left, right) | VarType::Or(left, right) => {
                let mut result = self.dep_query(left, num_struct_unwraps);
                result.append(self.dep_query(right, num_struct_unwraps));
                result
            }
        }
    }
}
