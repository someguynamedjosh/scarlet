use super::var_list::VarList;
use crate::{
    shared::{Item, ItemId, PrimitiveOperation, PrimitiveValue},
    stage4::structure::Environment,
    util::*,
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
        params: Vec<ItemId>,
        defined_in: Option<ItemId>,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        let param_vars = self.item_vars(&params, currently_computing.clone())?;
        self.flatten_type(typee, currently_computing.clone(), param_vars.as_slice())
    }

    pub fn type_of_is_same_variant(
        &mut self,
        base: ItemId,
        other: ItemId,
        defined_in: Option<ItemId>,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        let btype = self.compute_type(base, currently_computing.clone())?;
        let otype = self.compute_type(other, currently_computing.clone())?;
        let mut from_vars = VarList::new();
        from_vars.append(
            &self
                .get_from_variables(btype, currently_computing.clone())?
                .into_vec(),
        );
        from_vars.append(
            &self
                .get_from_variables(otype, currently_computing.clone())?
                .into_vec(),
        );
        MOk(self.with_from_vars(self.bool_type(), from_vars, defined_in))
    }

    pub fn type_of_primitive_operation(
        &mut self,
        op: PrimitiveOperation,
        defined_in: Option<ItemId>,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        let mut from_vars = VarList::new();
        let typee = self.op_type(&op);
        for input in op.inputs() {
            let input_type = self.compute_type(input, currently_computing.clone())?;
            let input_vars = self.get_from_variables(input_type, currently_computing.clone())?;
            from_vars.append(&input_vars.into_vec()[..]);
        }
        MOk(self.with_from_vars(typee, from_vars, defined_in))
    }

    pub fn type_of_primitive_value(&self, pv: &PrimitiveValue) -> MaybeResult<ItemId, String> {
        MOk(match pv {
            PrimitiveValue::Bool(..) => self.bool_type(),
            PrimitiveValue::I32(..) => self.i32_type(),
        })
    }

    pub fn type_of_type_is(
        &mut self,
        exact: bool,
        typee: ItemId,
        base: ItemId,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        if exact {
            MOk(typee)
        } else {
            self.compute_type(base, currently_computing)
        }
    }

    pub fn type_of_variable(
        &mut self,
        typee: ItemId,
        selff: ItemId,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        self.flatten_type(typee, currently_computing.clone(), &[selff])
    }
}
