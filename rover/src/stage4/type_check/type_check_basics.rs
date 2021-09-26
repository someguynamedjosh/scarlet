use crate::{
    shared::{Item, ItemId, Replacements},
    stage4::structure::Environment,
};

impl Environment {
    pub fn type_check_inductive_value(
        &self,
        typee: ItemId,
        variant_id: ItemId,
    ) -> Result<(), String> {
        if typee == self.god_type() {
            // Variants of the god type can be defined anywhere.
            Ok(())
        } else {
            let type_defined_in = &self.items[typee.0].defined_in.unwrap();
            let defining_scope = &self.items[type_defined_in.0].definition;
            if let Item::Defining { definitions, base } = defining_scope {
                let is_defined_here = definitions
                    .iter()
                    .any(|(_, defined)| *defined == variant_id);
                // The second condition is to check that this is not a general scope shared by
                // many things, but is instead one that is a defining construct on the type.
                if !is_defined_here || *base != typee {
                    todo!("nice error, variant defined outside of defining construct on type")
                } else {
                    Ok(())
                }
            } else {
                todo!("nice error, variant defined outside of defining construct on type")
            }
        }
    }

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
            // Deliberately keep any From constructs.
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
