use crate::{shared::ItemId, stage4::structure::Environment};

impl Environment {
    fn type_check_condition(&self, cond: ItemId) -> Result<(), String> {
        let cond_type = self.item_base_type(cond);
        if !self.are_def_equal(cond_type, self.bool_type()) {
            todo!("nice error, condition is not a Boolean");
        }
        Ok(())
    }

    fn type_check_elif_clauses(
        &self,
        value_type: ItemId,
        elif_clauses: &Vec<(ItemId, ItemId)>,
    ) -> Result<(), String> {
        for (cond, value) in elif_clauses {
            self.type_check_condition(*cond)?;
            let this_value_type = self.item_base_type(*value);
            if !self.are_def_equal(value_type, this_value_type) {
                todo!("nice error, elif value type does not match initial value type");
            }
        }
        Ok(())
    }

    fn type_check_else_clause(
        &self,
        value_type: ItemId,
        else_clause: ItemId,
    ) -> Result<(), String> {
        let else_value_type = self.item_base_type(else_clause);
        if !self.are_def_equal(value_type, else_value_type) {
            todo!("nice error, else value type does not match initial value type");
        }
        Ok(())
    }

    pub fn type_check_pick(
        &self,
        initial_clause: (ItemId, ItemId),
        elif_clauses: &Vec<(ItemId, ItemId)>,
        else_clause: ItemId,
    ) -> Result<(), String> {
        let value_type = self.item_base_type(initial_clause.1);
        self.type_check_condition(initial_clause.0)?;
        self.type_check_elif_clauses(value_type, elif_clauses)?;
        self.type_check_else_clause(value_type, else_clause)?;
        Ok(())
    }
}
