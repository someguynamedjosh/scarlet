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

    pub fn type_of_inductive_type(
        &mut self,
        params: Vec<ItemId>,
        defined_in: Option<ItemId>,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        let mut from_vars = VarList::new();
        for param in params {
            let typee = self.compute_type(param, currently_computing.clone())?;
            let this_param_vars = self.get_from_variables(typee)?;
            from_vars.append(&this_param_vars.into_vec()[..]);
        }
        let typ = Item::FromType {
            base: self.god_type(),
            vars: from_vars.into_vec(),
        };
        MOk(self.insert(typ, defined_in))
    }

    pub fn type_of_inductive_value(
        &mut self,
        typee: ItemId,
        records: Vec<ItemId>,
        defined_in: Option<ItemId>,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        let type_type = self.compute_type(typee, currently_computing.clone())?;
        let mut from_vars = self.get_from_variables(type_type)?;
        for recorded in records {
            let typee = self.compute_type(recorded, currently_computing.clone())?;
            let recorded_vars = self.get_from_variables(typee)?;
            from_vars.append(&recorded_vars.into_vec()[..]);
        }
        MOk(self.with_from_vars(typee, from_vars, defined_in))
    }

    pub fn type_of_is_same_variant(
        &mut self,
        base: ItemId,
        other: ItemId,
        defined_in: Option<ItemId>,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        let btype = self.compute_type(base, currently_computing.clone())?;
        let otype = self.compute_type(other, currently_computing)?;
        let mut from_vars = VarList::new();
        from_vars.append(&self.get_from_variables(btype)?.into_vec());
        from_vars.append(&self.get_from_variables(otype)?.into_vec());
        MOk(self.with_from_vars(self.bool_type(), from_vars, defined_in))
    }

    pub fn type_of_primitive_operation(
        &mut self,
        op: PrimitiveOperation,
        defined_in: Option<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        let mut from_vars = VarList::new();
        let typee = self.op_type(&op);
        for input in op.inputs() {
            let input_type = self.items[input.0].typee.unwrap();
            let input_vars = self.get_from_variables(input_type)?;
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
        let base = typee;
        let type_type = self.compute_type(typee, currently_computing)?;
        let mut vars = self.get_from_variables(type_type)?;
        vars.push(selff);
        let vars = vars.into_vec();
        MOk(self.insert(Item::FromType { base, vars }, Some(selff)))
    }
}
