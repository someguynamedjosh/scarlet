use crate::{
    shared::{ItemId, Replacements},
    stage4::structure::Environment,
};

impl Environment {
    pub fn type_check_replacing(
        &self,
        item: ItemId,
        replacements: &Replacements,
    ) -> Result<(), String> {
        for (target, val) in replacements {
            let var_type = self.get_var_type(*target);
            let val_type = self.item_base_type(*val);
            if !self.are_def_equal_after_replacements(var_type, val_type, replacements) {
                return Err(format!(
                    "(at {:?}) {:?} and {:?} have differing types",
                    item, target, val
                ));
            }
        }
        Ok(())
    }

    pub fn type_check_is_same_variant(
        &self,
        item: ItemId,
        base: ItemId,
        other: ItemId,
    ) -> Result<(), String> {
        let btype = self.item_base_type(base);
        let otype = self.item_base_type(other);
        if !self.are_def_equal(btype, otype) {
            Err(format!(
                "(at {:?}) {:?} and {:?} have differing types",
                item, base, other,
            ))
        } else {
            Ok(())
        }
    }

    pub fn type_check_type_is(
        &self,
        exact: bool,
        base: ItemId,
        typee: ItemId,
    ) -> Result<(), String> {
        if exact {
            // Deliberately keep any From clauses.
            let base_type = self.items[base.0].typee.unwrap();
            if !self.are_def_equal(base_type, typee) {
                todo!("nice error, value does not match exact type assert")
            }
        } else {
            let base_type = self.item_base_type(base);
            let typee = self.after_from(typee);
            if !self.are_def_equal(base_type, typee) {
                todo!("nice error, value does not match type assert")
            }
        }
        Ok(())
    }
}
