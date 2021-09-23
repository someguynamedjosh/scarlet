use super::var_list::VarList;
use crate::{
    shared::{Item, ItemId, PrimitiveOperation, PrimitiveValue},
    stage4::structure::Environment,
};

impl Environment {
    pub fn op_type(&self, op: &PrimitiveOperation) -> ItemId {
        match op {
            PrimitiveOperation::I32Math(..) => self.i32_type(),
        }
    }

    pub fn type_of_inductive_value(
        &mut self,
        typee: ItemId,
        records: Vec<ItemId>,
    ) -> Result<ItemId, String> {
        let mut from_vars = VarList::new();
        for recorded in records {
            let typee = self.compute_type(recorded)?;
            let recorded_vars = self.get_from_variables(typee)?;
            from_vars.append(&recorded_vars.into_vec()[..]);
        }
        Ok(self.with_from_vars(typee, from_vars))
    }

    pub fn type_of_is_same_variant(
        &mut self,
        base: ItemId,
        other: ItemId,
    ) -> Result<ItemId, String> {
        let btype = self.compute_type(base)?;
        let otype = self.compute_type(other)?;
        let mut from_vars = VarList::new();
        from_vars.append(&self.get_from_variables(btype)?.into_vec());
        from_vars.append(&self.get_from_variables(otype)?.into_vec());
        Ok(self.with_from_vars(self.bool_type(), from_vars))
    }

    pub fn type_of_primitive_operation(
        &mut self,
        op: PrimitiveOperation,
    ) -> Result<ItemId, String> {
        let mut from_vars = VarList::new();
        let typee = self.op_type(&op);
        for input in op.inputs() {
            let input_type = self.items[input.0].typee.unwrap();
            let input_vars = self.get_from_variables(input_type)?;
            from_vars.append(&input_vars.into_vec()[..]);
        }
        Ok(self.with_from_vars(typee, from_vars))
    }

    pub fn type_of_primitive_value(&self, pv: &PrimitiveValue) -> Result<ItemId, String> {
        Ok(match pv {
            PrimitiveValue::Bool(..) => self.bool_type(),
            PrimitiveValue::I32(..) => self.i32_type(),
        })
    }

    pub fn type_of_type_is(
        &mut self,
        exact: bool,
        typee: ItemId,
        base: ItemId,
    ) -> Result<ItemId, String> {
        if exact {
            Ok(typee)
        } else {
            self.compute_type(base)
        }
    }

    pub fn type_of_variable(&mut self, typee: ItemId, selff: ItemId) -> Result<ItemId, String> {
        let base = typee;
        let vars = vec![selff];
        Ok(self.insert(Item::FromType { base, vars }))
    }
}
