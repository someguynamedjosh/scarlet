use super::var_list::VarList;
use crate::{shared::ItemId, stage4::structure::Environment};

impl Environment {
    pub fn type_of_pick(
        &mut self,
        initial_clause: (ItemId, ItemId),
        elif_clauses: Vec<(ItemId, ItemId)>,
        else_clause: ItemId,
        defined_in: Option<ItemId>,
    ) -> Result<ItemId, String> {
        let id = initial_clause.1;
        let initial_value_type = self.compute_type(id)?;
        // What type it is after all variables are replaced.
        let base_value_type = self.after_from(initial_value_type);

        let mut vars = VarList::new();
        {
            // Type check will ensure this is identical to the other types.
            let typ = self.compute_type(initial_clause.0)?;
            vars.append(&self.get_from_variables(typ)?.into_vec());
            vars.append(&self.get_from_variables(initial_value_type)?.into_vec());
        }
        for (cond, val) in elif_clauses {
            let typ = self.compute_type(cond)?;
            vars.append(&self.get_from_variables(typ)?.into_vec());
            let typ = self.compute_type(val)?;
            vars.append(&self.get_from_variables(typ)?.into_vec());
        }
        {
            let typ = self.compute_type(else_clause)?;
            vars.append(&self.get_from_variables(typ)?.into_vec());
        }

        Ok(self.with_from_vars(base_value_type, vars, defined_in))
    }
}
