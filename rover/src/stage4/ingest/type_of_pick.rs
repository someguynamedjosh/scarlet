use super::var_list::VarList;
use crate::{shared::ItemId, stage4::structure::Environment, util::*};

impl Environment {
    pub fn type_of_pick(
        &mut self,
        initial_clause: (ItemId, ItemId),
        elif_clauses: Vec<(ItemId, ItemId)>,
        else_clause: ItemId,
        defined_in: Option<ItemId>,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        let id = initial_clause.1;
        let initial_value_type = self.compute_type(id, currently_computing.clone())?;
        // What type it is after all variables are replaced.
        let base_value_type = self.after_from(initial_value_type);

        let mut vars = VarList::new();
        {
            let typ = self.compute_type(initial_clause.0, currently_computing.clone())?;
            vars.append(&self.get_from_variables(typ)?.into_vec());
            vars.append(&self.get_from_variables(initial_value_type)?.into_vec());
        }
        for (cond, val) in elif_clauses {
            // Type check will ensure this is identical to the other types.
            let typ = self.compute_type(cond, currently_computing.clone())?;
            vars.append(&self.get_from_variables(typ)?.into_vec());
            let typ = self.compute_type(val, currently_computing.clone())?;
            vars.append(&self.get_from_variables(typ)?.into_vec());
        }
        {
            let typ = self.compute_type(else_clause, currently_computing)?;
            vars.append(&self.get_from_variables(typ)?.into_vec());
        }

        MOk(self.with_from_vars(base_value_type, vars, defined_in))
    }
}
