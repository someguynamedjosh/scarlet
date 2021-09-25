use super::var_list::VarList;
use crate::{shared::ItemId, stage4::structure::Environment, util::*};

impl Environment {
    fn process_condition(
        &mut self,
        condition: ItemId,
        tentative: &mut bool,
        vars: &mut VarList,
        currently_computing: &Vec<ItemId>,
    ) -> Result<(), String> {
        let typee = self.compute_type(condition, currently_computing.clone());
        let typee = typee.into_option_or_err()?;
        if let Some(typee) = typee {
            vars.append(&self.get_from_variables(typee)?.into_vec());
        } else {
            *tentative = true;
        }
        Ok(())
    }

    fn process_value(
        &mut self,
        value: ItemId,
        tentative: &mut bool,
        base_type: &mut Option<ItemId>,
        vars: &mut VarList,
        currently_computing: &Vec<ItemId>,
    ) -> Result<(), String> {
        let typee = self.compute_type(value, currently_computing.clone());
        let typee = typee.into_option_or_err()?;
        if let Some(typee) = typee {
            *base_type = Some(self.after_from(typee));
            vars.append(&self.get_from_variables(typee)?.into_vec());
        } else {
            *tentative = true;
        }
        Ok(())
    }

    pub fn type_of_pick(
        &mut self,
        initial_clause: (ItemId, ItemId),
        elif_clauses: Vec<(ItemId, ItemId)>,
        else_clause: ItemId,
        defined_in: Option<ItemId>,
        cur_comp: Vec<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        let mut tentative = false;
        let mut base_type = None;
        let mut vars = VarList::new();

        self.process_condition(initial_clause.0, &mut tentative, &mut vars, &cur_comp)?;
        let val = initial_clause.1;
        self.process_value(val, &mut tentative, &mut base_type, &mut vars, &cur_comp)?;

        for (cond, val) in elif_clauses {
            self.process_condition(cond, &mut tentative, &mut vars, &cur_comp)?;
            self.process_value(val, &mut tentative, &mut base_type, &mut vars, &cur_comp)?;
        }
        let val = else_clause;
        self.process_value(val, &mut tentative, &mut base_type, &mut vars, &cur_comp)?;

        if let Some(typee) = base_type {
            MOk(self.with_from_vars(typee, vars, defined_in))
        } else {
            MNone
        }
    }
}
