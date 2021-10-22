use crate::{
    shared::OrderedMap,
    stage2::structure::{
        Condition, Definition, Environment, ItemId, StructField, Substitution, VariableId,
    },
};

impl<'x> Environment<'x> {
    pub(super) fn substitute(
        &mut self,
        original: ItemId<'x>,
        substitutions: &OrderedMap<VariableId<'x>, ItemId<'x>>,
    ) -> Option<ItemId<'x>> {
        if self.query_stack_contains(original) && self.get_deps(original).len() > 0 {
            println!("{:#?}", self);
            println!("Early exit from {:?}", original);
            None
        } else {
            println!("Substituting {:?} in {:?}", substitutions, original);
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
            Definition::Other(..) => unreachable!(),
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
            Definition::Substitute(base, original_subs) => {
                let mut subs_for_base = OrderedMap::new();
                for &(target, value) in substitutions {
                    if original_subs
                        .iter()
                        .any(|sub| self.item_as_variable(sub.target.unwrap()) == target)
                    {
                        continue;
                    } else {
                        subs_for_base.insert_no_replace(target, value)
                    }
                }
                let base = if subs_for_base.len() > 0 {
                    self.substitute(base, &subs_for_base)?
                } else {
                    base
                };
                let original_subs = original_subs
                    .into_iter()
                    .map(|sub| {
                        Some(Substitution {
                            target: sub.target,
                            value: self.substitute(sub.value, substitutions)?,
                        })
                    })
                    .collect::<Option<_>>()?;
                let def = Definition::Substitute(base, original_subs);
                self.item_with_new_definition(original, def, true)
            }
            Definition::Variable(var_id) => {
                if let Some(sub) = substitutions.get(&var_id) {
                    *sub
                } else {
                    original
                }
            }
        })
    }
}
