use crate::{
    shared::{Item, ItemId, PrimitiveOperation, PrimitiveValue},
    stage4::structure::Environment,
    util::*,
};

impl Environment {
    pub fn compute_type(
        &mut self,
        of: ItemId,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<ItemId, String> {
        assert!(of.0 < self.items.len());
        if currently_computing.contains(&of) {
            return MNone;
        }
        let item = &self.items[of.0];
        if let Some(typee) = item.typee {
            return MOk(typee);
        }
        let item = &self.items[of.0];
        let defined_in = item.defined_in;
        let base_type = self.compute_base_type(of, currently_computing.clone());
        let new_computing = [currently_computing, vec![of]].concat();
        let dependencies = self.compute_dependencies(of, new_computing)?;
        let typee = self.type_with_dependencies(base_type, dependencies, defined_in);
        self.set_type(of, typee);
        MOk(typee)
    }

    fn compute_base_type(&mut self, of: ItemId, mut currently_computing: Vec<ItemId>) -> ItemId {
        let item = &self.items[of.0];
        currently_computing.push(of);
        match &item.definition {
            Item::Defining { base, .. } => {
                let base = *base;
                self.compute_base_type(base, currently_computing)
            }
            Item::FromType { base, .. } => {
                let base = *base;
                self.compute_base_type(base, currently_computing)
            }
            Item::GodType { .. } => self.god_type(),
            Item::VariantInstance { typee, .. } => self.after_from(*typee),
            Item::IsSameVariant { .. } => self.bool_type(),
            Item::Pick {
                initial_clause,
                elif_clauses,
                else_clause,
            } => {
                todo!()
            }
            Item::PrimitiveOperation(op) => match op {
                PrimitiveOperation::I32Math(..) => self.i32_type(),
            },
            Item::PrimitiveType(..) => self.god_type(),
            Item::PrimitiveValue(pv) => match pv {
                PrimitiveValue::Bool(..) => self.bool_type(),
                PrimitiveValue::I32(..) => self.i32_type(),
            },
            Item::Replacing { base, .. } => {
                let base = *base;
                self.compute_base_type(base, currently_computing)
            }
            Item::TypeIs { exact, typee, base } => {
                todo!()
            }
            Item::Variable { typee, .. } => self.after_from(*typee),
        }
    }
}
