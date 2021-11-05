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
        if substitutions.len() == 0 {
            Some(original)
        } else if self.query_stack_contains(original) && self.get_deps(original).len() > 0 {
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
        let new_def = match self.items[original].definition.clone().unwrap() {
            Definition::BuiltinOperation(op, args) => {
                let args = args
                    .into_iter()
                    .map(|i| self.substitute(i, substitutions))
                    .collect::<Option<_>>()?;
                Definition::BuiltinOperation(op, args)
            }
            Definition::BuiltinValue(..) => return Some(original),
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
                Definition::Match {
                    base,
                    conditions,
                    else_value,
                }
            }
            Definition::Member(base, name) => {
                let base = self.substitute(base, substitutions)?;
                Definition::Member(base, name)
            }
            Definition::Other(id) => return self.substitute_impl(id, substitutions),
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
                Definition::ResolvedSubstitute(base, original_subs)
            }
            Definition::SetEager { base, vals, eager } => {
                let base = self.substitute(base, substitutions)?;
                let mut base_deps = self.get_deps(base);
                for &val in &vals {
                    for (dep, _) in self.get_deps(val) {
                        base_deps.remove(&dep);
                    }
                }
                let has_any_deps_at_all = !base_deps.is_empty();
                if has_any_deps_at_all {
                    Definition::SetEager { base, vals, eager }
                } else {
                    return Some(base);
                }
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
                Definition::Struct(fields)
            }
            Definition::UnresolvedSubstitute(..) => unreachable!(),
            Definition::Variable { var, typee } => {
                if let Some(sub) = substitutions.get(&var) {
                    return Some(*sub);
                } else {
                    let typee = self.substitute_var_type(typee, substitutions)?;
                    Definition::Variable { var, typee }
                }
            }
        };
        Some(self.item_with_new_definition(original, new_def, true))
    }

    pub fn substitute_var_type(
        &mut self,
        typee: VarType<'x>,
        substitutions: &Substitutions<'x>,
    ) -> Option<VarType<'x>> {
        let typee = match typee {
            VarType::God | VarType::_32U | VarType::Bool => typee,
            VarType::Just(other) => VarType::Just(self.substitute(other, substitutions)?),
            VarType::And(l, r) => VarType::And(
                self.substitute(l, substitutions)?,
                self.substitute(r, substitutions)?,
            ),
            VarType::Or(l, r) => VarType::Or(
                self.substitute(l, substitutions)?,
                self.substitute(r, substitutions)?,
            ),
        };
        Some(typee)
    }
}
