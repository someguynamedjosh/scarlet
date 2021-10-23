use crate::stage2::structure::{Definition, Environment, ItemId};

impl<'x> Environment<'x> {
    fn reduce_definition(&mut self, def: Definition<'x>) -> Definition<'x> {
        match def {
            Definition::After { .. } => todo!(),
            Definition::BuiltinOperation(op, args) => self.reduce_builtin_op(op, args),
            Definition::BuiltinValue(..) => def,
            Definition::Match { .. } => unreachable!(),
            Definition::Member(..) => unreachable!(),
            Definition::Other(..) => unreachable!(),
            Definition::Struct(fields) => self.reduce_struct(fields),
            Definition::Substitute(..) => unreachable!(),
            Definition::Variable(..) => def,
        }
    }

    fn reduce_from_scratch(&mut self, original: ItemId<'x>) -> ItemId<'x> {
        let definition = self.items[original].definition.clone().unwrap();
        match definition {
            Definition::Match {
                base,
                conditions,
                else_value,
            } => self.reduce_match(base, else_value, conditions, original),
            Definition::Member(base, member) => self.reduce_member(base, member),
            Definition::Other(item) => self.reduce(item),
            Definition::Substitute(base, subs) => self.reduce_substitution(subs, base, original),
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
            let result =
                self.with_query_stack_frame(original, |this| this.reduce_from_scratch(original));
            self.items[original].cached_reduction = Some(result);
            self.get_deps(original);
            // println!("{:#?}", self);
            // println!("{:?} becomes {:?}", original, result);
            assert!(self.get_deps(result).len() <= self.get_deps(original).len());
            assert_eq!(self.reduce(result), result);
            result
        }
    }
}
