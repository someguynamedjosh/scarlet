use super::structure::{BuiltinValue, Definition, Environment, ItemId};
use crate::stage2::structure::{BuiltinOperation, StructField};

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
            } => todo!(),
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
            Definition::Substitute(_, _) => todo!(),
            Definition::Variable(..) => def,
        }
    }

    pub fn reduce(&mut self, original: ItemId<'x>) -> ItemId<'x> {
        if let Some(reduction) = &self.items[original].cached_reduction {
            *reduction
        } else {
            let definition = self.items[original].definition.clone().unwrap();
            let reduced_definition = self.reduce_definition(definition);
            let mut reduced_item = self.items[original].clone();
            reduced_item.definition = Some(reduced_definition);
            let result = self.items.get_or_push(reduced_item);
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
