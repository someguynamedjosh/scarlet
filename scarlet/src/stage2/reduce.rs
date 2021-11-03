use super::structure::{Substitutions, UnresolvedSubstitution};
use crate::{
    shared::OrderedSet,
    stage2::{
        matchh::MatchResult,
        structure::{
            BuiltinOperation, BuiltinValue, Definition, Environment, ItemId, StructField, VarType,
            VariableInfo,
        },
    },
};

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
            // println!("{:?} becomes {:?}", original, result);
            assert!(self.get_deps(result).len() <= self.get_deps(item).len());
            // println!("{:#?}", self);
            assert_eq!(self.reduce(result), result);
            result
        }
    }

    fn reduce_from_scratch(&mut self, item: ItemId<'x>) -> ItemId<'x> {
        let definition = self.items[item].definition.clone().unwrap();
        match definition {
            Definition::Other(other) => self.reduce(other),
            Definition::ResolvedSubstitute(base, subs) => self.substitute(base, &subs).unwrap(),
            Definition::UnresolvedSubstitute(_, _) => {
                self.resolve_substitution(item);
                self.reduce_from_scratch(item)
            }
            _ => {
                let reduced_definition = self.reduce_definition(definition);
                self.item_with_new_definition(item, reduced_definition, false)
            }
        }
    }

    fn reduce_definition(&mut self, def: Definition<'x>) -> Definition<'x> {
        match def.clone() {
            Definition::Other(_) => unreachable!(),
            Definition::ResolvedSubstitute(..) => unreachable!(),
            Definition::UnresolvedSubstitute(..) => unreachable!(),

            Definition::BuiltinOperation(op, args) => match op {
                BuiltinOperation::Sum32U => {
                    if let Some(args) = self.args_as_builtin_values(&args[..]) {
                        Definition::BuiltinValue(BuiltinValue::_32U(
                            args[0].unwrap_32u() + args[1].unwrap_32u(),
                        ))
                    } else {
                        def
                    }
                }
                BuiltinOperation::Dif32U => {
                    if let Some(args) = self.args_as_builtin_values(&args[..]) {
                        Definition::BuiltinValue(BuiltinValue::_32U(
                            args[0].unwrap_32u() - args[1].unwrap_32u(),
                        ))
                    } else {
                        def
                    }
                }
            },
            Definition::BuiltinValue(..) => def,
            Definition::Match {
                base,
                conditions,
                else_value,
            } => todo!(),
            Definition::Member(_, _) => todo!(),
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
                };
                Definition::Variable { var, typee }
            }
        }
    }
}