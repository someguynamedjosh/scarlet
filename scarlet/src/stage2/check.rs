use super::structure::Member;
use crate::stage2::{
    construct::constructs::{CBuiltinValue, COperation, Operation},
    structure::{BuiltinOperation, BuiltinValue, ConstructId, Environment, VarType},
};

impl<'x> Environment<'x> {
    fn index_in_bounds_theorem(&mut self, index: ConstructId<'x>, length: u32) -> ConstructId<'x> {
        let length = self.push_con(CBuiltinValue(BuiltinValue::_32U(length)));
        let in_bounds = self.push_con(COperation {
            op: Operation::LessThan32U,
            args: vec![index, length],
        });
        let in_bounds = self.reduce(in_bounds);
        let truee = self.push_con(CBuiltinValue(BuiltinValue::Bool(true)));
        let theorem_var_type = VarType::And(in_bounds, truee);
        self.push_var(theorem_var_type)
    }

    fn check_index(
        &mut self,
        index: ConstructId<'x>,
        t_index_in_range: ConstructId<'x>,
        base_bounding_pattern: ConstructId<'x>,
    ) -> bool {
        todo!()
    }

    pub fn check(&mut self, item: ConstructId<'x>) {
        // let pattern_bool = self.get_or_push_var(VarType::Bool);
        let pattern_32u = self.get_or_push_var(VarType::_32U);
        todo!()
    }
}
