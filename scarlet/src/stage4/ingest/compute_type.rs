use crate::{
    shared::{BuiltinOperation, Item, ItemId, PrimitiveValue},
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
        let base_type = self.compute_base_type(of, currently_computing.clone())?;
        let dependencies = self.compute_dependencies(of, currently_computing)?;
        let typee = self.type_with_dependencies(base_type, dependencies, defined_in);
        self.set_type(of, typee);
        MOk(typee)
    }

    fn compute_base_type(
        &mut self,
        of: ItemId,
        mut currently_computing: Vec<ItemId>,
    ) -> Option<ItemId> {
        let item = &self.items[of.0];
        if currently_computing.contains(&of) {
            return None;
        }
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
            Item::GodType { .. } => Some(self.god_type()),
            Item::VariantInstance { typee, .. } => Some(self.after_from(*typee)),
            Item::Pick {
                initial_clause,
                elif_clauses,
                else_clause,
            } => {
                let (ic, ec) = (*initial_clause, *else_clause);
                let eic = elif_clauses.clone();
                self.compute_pick_base_type(ic, eic, ec, currently_computing)
            }
            Item::BuiltinOperation(op) => Some(match op {
                BuiltinOperation::I32Math(..) => self.i32_type(),
                BuiltinOperation::AreSameVariant { base: _, other: _ } => self.bool_type(),
                BuiltinOperation::Reinterpret { new_type, .. } => *new_type,
            }),
            Item::PrimitiveType(..) => Some(self.god_type()),
            Item::PrimitiveValue(pv) => Some(match pv {
                PrimitiveValue::Bool(..) => self.bool_type(),
                PrimitiveValue::I32(..) => self.i32_type(),
            }),
            Item::Replacing { base, .. } => {
                let base = *base;
                self.compute_base_type(base, currently_computing)
            }
            Item::TypeIs {
                exact: _,
                typee: _,
                base: _,
            } => {
                todo!()
            }
            Item::Variable { typee, .. } => Some(self.after_from(*typee)),
        }
    }

    fn compute_pick_base_type(
        &mut self,
        initial_clause: (ItemId, ItemId),
        elif_clauses: Vec<(ItemId, ItemId)>,
        else_clause: ItemId,
        currently_computing: Vec<ItemId>,
    ) -> Option<ItemId> {
        if let Some(bt) = self.compute_base_type(initial_clause.1, currently_computing.clone()) {
            Some(bt)
        } else if let Some(bt) = self.compute_base_type(else_clause, currently_computing.clone()) {
            Some(bt)
        } else {
            for (_, val) in elif_clauses {
                if let Some(bt) = self.compute_base_type(val, currently_computing.clone()) {
                    return Some(bt);
                }
            }
            None
        }
    }
}
