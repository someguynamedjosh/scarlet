use super::structure::{BuiltinValue, Condition, Definition, Environment, ItemId, VariableId};
use crate::{
    shared::{OrderedMap, OrderedSet},
    stage2::structure::{BuiltinOperation, StructField},
};

impl<'x> Environment<'x> {
    fn args_as_builtin_values(&mut self, args: &[ItemId<'x>]) -> Option<Vec<BuiltinValue>> {
        let mut result = Vec::new();
        for arg in args {
            let arg = self.reduce(*arg);
            if let Definition::BuiltinValue(value) = self.items[arg].definition.as_ref().unwrap() {
                result.push(*value);
            } else {
                return None;
            }
        }
        Some(result)
    }

    fn item_with_new_definition(
        &mut self,
        original: ItemId<'x>,
        new_def: Definition<'x>,
        wipe_deps_and_replacement: bool,
    ) -> ItemId<'x> {
        let mut new_item = self.items[original].clone();
        new_item.definition = Some(new_def);
        if wipe_deps_and_replacement {
            new_item.dependencies = None;
            new_item.cached_reduction = None;
        }
        self.items.get_or_push(new_item)
    }

    fn substitute(
        &mut self,
        original: ItemId<'x>,
        substitutions: &OrderedMap<VariableId<'x>, ItemId<'x>>,
    ) -> ItemId<'x> {
        match self.items[original].definition.clone().unwrap() {
            Definition::BuiltinOperation(op, args) => {
                let args = args
                    .into_iter()
                    .map(|i| self.substitute(i, substitutions))
                    .collect();
                let def = Definition::BuiltinOperation(op, args);
                self.item_with_new_definition(original, def, true)
            }
            Definition::BuiltinValue(..) => original,
            Definition::Match {
                base,
                conditions,
                else_value,
            } => todo!(),
            Definition::Member(base, name) => {
                let base = self.substitute(base, substitutions);
                let def = Definition::Member(base, name);
                self.item_with_new_definition(original, def, true)
            }
            Definition::Other(..) => unreachable!(),
            Definition::Struct(fields) => {
                let fields = fields
                    .into_iter()
                    .map(|f| {
                        let name = f.name;
                        let value = self.substitute(f.value, substitutions);
                        StructField { name, value }
                    })
                    .collect();
                let def = Definition::Struct(fields);
                self.item_with_new_definition(original, def, true)
            }
            Definition::Substitute(_, _) => todo!(),
            Definition::Variable(var_id) => {
                if let Some(sub) = substitutions.get(&var_id) {
                    *sub
                } else {
                    original
                }
            }
        }
    }

    pub fn item_as_variable(&self, item: ItemId<'x>) -> VariableId<'x> {
        match self.items[item].definition.as_ref().unwrap() {
            Definition::Member(_, _) => todo!(),
            Definition::Other(..) => unreachable!(),
            Definition::Variable(id) => *id,
            _ => todo!("Nice error, {:?} is not a variable", item),
        }
    }

    fn reduce_definition(&mut self, def: Definition<'x>) -> Definition<'x> {
        match def {
            Definition::BuiltinOperation(op, args) => match op {
                BuiltinOperation::Sum32U => {
                    if let Some(arg_values) = self.args_as_builtin_values(&args[..]) {
                        Definition::BuiltinValue(BuiltinValue::_32U(
                            arg_values[0].unwrap_32u() + arg_values[1].unwrap_32u(),
                        ))
                    } else {
                        Definition::BuiltinOperation(op, args)
                    }
                }
                BuiltinOperation::Dif32U => {
                    if let Some(arg_values) = self.args_as_builtin_values(&args[..]) {
                        Definition::BuiltinValue(BuiltinValue::_32U(
                            arg_values[0].unwrap_32u() - arg_values[1].unwrap_32u(),
                        ))
                    } else {
                        Definition::BuiltinOperation(op, args)
                    }
                }
                BuiltinOperation::_32UPattern => Definition::BuiltinOperation(op, args),
            },
            Definition::BuiltinValue(..) => def,
            Definition::Match {
                base,
                conditions,
                else_value,
            } => {
                unreachable!()
            }
            Definition::Member(_, _) => todo!(),
            Definition::Other(_) => todo!(),
            Definition::Struct(fields) => {
                let new_fields = fields
                    .into_iter()
                    .map(|field| {
                        let name = field.name;
                        let value = self.reduce(field.value);
                        StructField { name, value }
                    })
                    .collect();
                Definition::Struct(new_fields)
            }
            Definition::Substitute(..) => unreachable!(),
            Definition::Variable(..) => def,
        }
    }

    fn matches(&mut self, pattern: ItemId<'x>, value: ItemId<'x>) -> Option<bool> {
        match self.items[pattern].definition.as_ref().unwrap() {
            Definition::BuiltinOperation(_, _) => todo!(),
            Definition::BuiltinValue(pv) => match self.items[value].definition.as_ref().unwrap() {
                Definition::BuiltinValue(vv) => Some(pv == vv),
                Definition::Struct(..) => Some(false),
                _ => None,
            },
            Definition::Match { .. } => None,
            Definition::Member(_, _) => todo!(),
            Definition::Other(..) => unreachable!(),
            Definition::Struct(_) => todo!(),
            Definition::Substitute(..) => None,
            Definition::Variable(var) => {
                let var_pattern = self.vars[*var].pattern;
                self.matches(var_pattern, value)
            }
        }
    }

    fn reduce_from_scratch(&mut self, original: ItemId<'x>) -> ItemId<'x> {
        let definition = self.items[original].definition.clone().unwrap();
        match definition {
            Definition::Match {
                base,
                conditions,
                else_value,
            } => {
                let base = self.reduce(base);
                let mut new_conditions = Vec::new();
                let mut else_value = else_value;
                for condition in conditions {
                    let pattern = self.reduce(condition.pattern);
                    // Don't reduce yet as that might lead to needless infinite recursion.
                    let value = condition.value;
                    match self.matches(pattern, base) {
                        Some(true) => {
                            // If the match is always true, no need to evaluate further conditions.
                            // This should always be used when the previous conditions fail.
                            else_value = condition.value;
                            break;
                        }
                        // If the match will never occur, skip it.
                        Some(false) => (),
                        // If the match might occur, save it for later.
                        None => new_conditions.push(Condition { pattern, value }),
                    }
                }
                if new_conditions.len() == 0 {
                    self.reduce(else_value)
                } else {
                    let def = Definition::Match {
                        base,
                        conditions: new_conditions,
                        else_value,
                    };
                    self.item_with_new_definition(original, def, true)
                }
            }
            Definition::Substitute(base, subs) => {
                let mut final_subs = OrderedMap::new();
                for sub in subs {
                    let target = self.item_as_variable(sub.target.unwrap());
                    final_subs.insert_no_replace(target, sub.value);
                }
                let base = self.reduce(base);
                let subbed = self.substitute(base, &final_subs);
                println!("{:?} with {:?} becomes {:?}", base, final_subs, subbed);
                if subbed == base {
                    subbed
                } else {
                    self.reduce(subbed)
                }
            }
            _ => {
                let reduced_definition = self.reduce_definition(definition);
                self.item_with_new_definition(original, reduced_definition, false)
            }
        }
    }

    pub fn reduce(&mut self, original: ItemId<'x>) -> ItemId<'x> {
        if let Some(reduction) = &self.items[original].cached_reduction {
            *reduction
        } else {
            let result = self.reduce_from_scratch(original);
            self.items[original].cached_reduction = Some(result);
            self.reduce(result);
            debug_assert_eq!(self.reduce(result), result);
            result
        }
    }

    pub fn reduce_all(&mut self) {
        let id = if let Some((id, _)) = self.items.iter().next() {
            id
        } else {
            return;
        };
        self.reduce(id);
        let mut id = id;
        while let Some(next) = self.items.next(id) {
            id = next;
            self.reduce(id);
        }
    }
}
