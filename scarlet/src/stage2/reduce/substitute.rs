use crate::{
    shared::{OrderedMap, OrderedSet},
    stage2::structure::{
        Condition, Definition, Environment, ItemId, StructField, Substitutions, VarType, VariableId,
    },
};

impl<'x> Environment<'x> {
    pub fn substitute(
        &mut self,
        original: ItemId<'x>,
        substitutions: &Substitutions<'x>,
    ) -> Option<ItemId<'x>> {
        if self.query_stack_contains(original) && self.get_deps(original).len() > 0 {
            None
        } else {
            let result = self.with_query_stack_frame(original, |this| {
                this.substitute_impl(original, substitutions)
            });
            result
        }
    }

    pub(super) fn substitute_impl(
        &mut self,
        original: ItemId<'x>,
        substitutions: &OrderedMap<VariableId<'x>, ItemId<'x>>,
    ) -> Option<ItemId<'x>> {
        Some(match self.items[original].definition.clone().unwrap() {
            Definition::After { base, vals } => {
                let base = self.substitute(base, substitutions)?;
                let vals = vals
                    .into_iter()
                    .map(|i| self.substitute(i, substitutions))
                    .collect::<Option<_>>()?;
                let def = Definition::After { base, vals };
                self.item_with_new_definition(original, def, true)
            }
            Definition::BuiltinOperation(op, args) => {
                let args = args
                    .into_iter()
                    .map(|i| self.substitute(i, substitutions))
                    .collect::<Option<_>>()?;
                let def = Definition::BuiltinOperation(op, args);
                self.item_with_new_definition(original, def, true)
            }
            Definition::BuiltinValue(..) => original,
            Definition::Match {
                base,
                conditions,
                else_value,
            } => {
                let base = self.substitute(base, substitutions)?;
                let else_value = self.substitute(else_value, substitutions)?;
                let conditions = conditions
                    .into_iter()
                    .map(|c| {
                        Some(Condition {
                            pattern: self.substitute(c.pattern, substitutions)?,
                            value: self.substitute(c.value, substitutions)?,
                        })
                    })
                    .collect::<Option<_>>()?;
                let def = Definition::Match {
                    base,
                    conditions,
                    else_value,
                };
                self.item_with_new_definition(original, def, true)
            }
            Definition::Member(base, name) => {
                let base = self.substitute(base, substitutions)?;
                let def = Definition::Member(base, name);
                self.item_with_new_definition(original, def, true)
            }
            Definition::Other(id) => self.substitute_impl(id, substitutions)?,
            Definition::ResolvedSubstitute(base, original_subs) => {
                // The substitutions that we are currently doing that should be
                // applied to the base, because $original_subs does not override
                // them.
                let mut subs_for_base = OrderedMap::new();
                'outer_subs: for &(target, value) in substitutions {
                    for &(orsub_target, _) in &original_subs {
                        if orsub_target == target {
                            continue 'outer_subs;
                        }
                    }
                    subs_for_base.insert_no_replace(target, value)
                }
                let base = if subs_for_base.len() > 0 {
                    self.substitute(base, &subs_for_base)?
                } else {
                    base
                };
                let original_subs = original_subs
                    .into_iter()
                    .map(|(target, value)| Some((target, self.substitute(value, substitutions)?)))
                    .collect::<Option<_>>()?;
                let def = Definition::ResolvedSubstitute(base, original_subs);
                self.item_with_new_definition(original, def, true)
            }
            Definition::Struct(fields) => {
                let fields = fields
                    .into_iter()
                    .map(|f| {
                        let name = f.name;
                        let value = self.substitute(f.value, substitutions)?;
                        Some(StructField { name, value })
                    })
                    .collect::<Option<_>>()?;
                let def = Definition::Struct(fields);
                self.item_with_new_definition(original, def, true)
            }
            Definition::UnresolvedSubstitute(..) => unreachable!(),
            Definition::Variable {
                var,
                typee,
                consume,
            } => {
                if let Some(sub) = substitutions.get(&var) {
                    *sub
                } else {
                    // let typee = self.substitute(typee, substitutions)?;
                    // let def = Definition::Variable { var, typee };
                    // self.item_with_new_definition(original, def, true)
                    todo!()
                }
            }
        })
    }
}
