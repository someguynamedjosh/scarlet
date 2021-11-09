use crate::stage2::structure::{Definition, Environment, ItemId, StructField, VarType};

impl<'x> Environment<'x> {
    pub fn reduce(&mut self, item: ItemId<'x>) -> ItemId<'x> {
        if let Some(reduction) = &self.items[item].cached_reduction {
            *reduction
        } else if self.query_stack_contains(item) {
            println!("{:#?}", self);
            todo!("Handle recursive reduction on {:?}", item)
        } else {
            let result = self.with_query_stack_frame(item, |this| this.reduce_from_scratch(item));
            self.items[item].cached_reduction = Some(result);
            self.get_deps(item);
            self.get_deps(result);
            // println!("{:#?}", self);
            // println!("{:?} becomes {:?}", item, result);
            assert!(self.get_deps(result).len() <= self.get_deps(item).len());
            // println!("{:#?}", self);
            assert_eq!(self.reduce(result), result);
            result
        }
    }

    fn reduce_from_scratch(&mut self, item: ItemId<'x>) -> ItemId<'x> {
        let definition = self.items[item].definition.clone().unwrap();
        match definition {
            Definition::Match {
                base,
                conditions,
                else_value,
            } => self.reduce_match(base, else_value, conditions, item),
            Definition::Member(base, name) => self.reduce_member(base, name, item),
            Definition::Unresolved { .. } => {
                let resolved_item = self.resolve(item);
                if resolved_item == item {
                    self.reduce_from_scratch(resolved_item)
                } else {
                    self.reduce(resolved_item)
                }
            }
            Definition::Substitute(base, subs) => self.reduce_substitution(base, subs, item),
            _ => {
                let reduced_definition = self.reduce_definition(definition);
                self.item_with_new_definition(item, reduced_definition, false)
            }
        }
    }

    fn reduce_definition(&mut self, def: Definition<'x>) -> Definition<'x> {
        match def.clone() {
            Definition::Unresolved { .. } => unreachable!(),
            Definition::Substitute(..) => unreachable!(),

            Definition::BuiltinOperation(op, args) => self.reduce_builtin_op(def, op, args),
            Definition::BuiltinValue(..) => def,
            Definition::Match { .. } => unreachable!(),
            Definition::Member(..) => unreachable!(),
            Definition::SetEager { base, vals, eager } => {
                let base = self.reduce(base);
                let vals = vals.into_iter().map(|x| self.reduce(x)).collect();
                Definition::SetEager { base, vals, eager }
            }
            Definition::Struct(fields) => {
                let mut reduced_fields = Vec::new();
                for field in fields {
                    let name = field.name;
                    let value = self.reduce(field.value);
                    reduced_fields.push(StructField { name, value })
                }
                Definition::Struct(reduced_fields)
            }
            Definition::Variable { var, typee } => {
                let typee = match typee {
                    VarType::Bool | VarType::God | VarType::_32U => typee,
                    VarType::Just(other) => VarType::Just(self.reduce(other)),
                    VarType::And(l, r) => VarType::And(self.reduce(l), self.reduce(r)),
                    VarType::Or(l, r) => VarType::Or(self.reduce(l), self.reduce(r)),
                };
                Definition::Variable { var, typee }
            }
        }
    }
}
