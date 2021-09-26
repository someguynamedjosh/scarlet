use crate::{
    shared::{IntegerMathOperation, Item, ItemId, PrimitiveOperation, Replacements},
    stage4::{ingest::VarList, structure::Environment},
    util::*,
};

impl Environment {
    pub fn compute_dependencies(
        &mut self,
        item: ItemId,
        currently_computing: Vec<ItemId>,
    ) -> MaybeResult<VarList, String> {
        match &self.items[item.0].definition {
            Item::Defining { base, .. } => {
                let base = *base;
                self.compute_dependencies(base, currently_computing)
            }
            Item::FromType { base, values } => {
                let base = *base;
                let values = values.clone();
                self.from_type_dependencies(base, values, currently_computing)
            }
            Item::GodType | Item::PrimitiveType(..) | Item::PrimitiveValue(..) => {
                MOk(VarList::new())
            }
            Item::VariantInstance {
                typee,
                values: params,
                ..
            } => {
                let typee = *typee;
                let values = params.clone();
                self.variant_instance_dependencies(typee, values, currently_computing)
            }
            Item::IsSameVariant { base, other } => {
                let base = *base;
                let other = *other;
                self.is_same_variant_dependencies(base, other, currently_computing)
            }
            Item::Pick {
                initial_clause,
                elif_clauses,
                else_clause,
            } => {
                todo!()
            }
            Item::PrimitiveOperation(op) => {
                let op = op.clone();
                self.primitive_op_dependencies(op, currently_computing)
            }
            Item::Replacing {
                base,
                replacements,
                unlabeled_replacements,
            } => {
                let base = *base;
                let replacements = replacements.clone();
                let unlabeled_replacements = unlabeled_replacements.clone();
                self.replacing_dependencies(
                    item,
                    base,
                    replacements,
                    unlabeled_replacements,
                    currently_computing,
                )
            }
            Item::TypeIs { base, .. } => {
                let base = *base;
                self.compute_dependencies(base, currently_computing)
            }
            Item::Variable { selff, typee } => {
                let selff = *selff;
                let typee = *typee;
                self.variable_dependencies(selff, typee, currently_computing)
            }
        }
    }

    pub fn type_with_dependencies(
        &mut self,
        mut base_type: ItemId,
        mut dependencies: VarList,
        defined_in: Option<ItemId>,
    ) -> ItemId {
        if dependencies.len() > 0 {
            if let Item::FromType {
                base: other_base,
                values: other_values,
            } = &self.items[base_type.0].definition
            {
                base_type = *other_base;
                let mut all_dependencies = VarList::from(other_values.clone());
                all_dependencies.append(dependencies.as_slice());
                dependencies = all_dependencies;
            }
            self.insert(
                Item::FromType {
                    base: base_type,
                    values: dependencies.into_vec(),
                },
                defined_in,
            )
        } else {
            base_type
        }
    }
}
