use std::marker::PhantomData;

use super::{
    construct::{
        constructs::{CUnresolved, CVariable},
        BoxedConstruct, Construct,
    },
    structure::{
        AnnotatedConstruct, BuiltinValue, ConstructId, Environment, Token, VarType, Variable,
    },
};
use crate::stage2::structure::Member;

impl<'x> Environment<'x> {
    pub fn get_definition(&self, of: ConstructId<'x>) -> &BoxedConstruct<'x> {
        self.items[of].definition.as_ref().unwrap()
    }

    pub fn get_resolved_definition(&mut self, of: ConstructId<'x>) -> &BoxedConstruct<'x> {
        let def = self.items[of].definition.as_ref().unwrap();
        todo!()
        // if let Definition::Unresolved(..) = def {
        //     let resolved = self.resolve(of);
        //     self.get_definition(resolved)
        // } else {
        //     self.items[of].definition.as_ref().unwrap()
        // }
    }

    pub(super) fn args_as_builtin_values(
        &mut self,
        args: &[ConstructId<'x>],
    ) -> Option<Vec<BuiltinValue>> {
        let mut result = Vec::new();
        for arg in args {
            let arg = self.reduce(*arg);
            todo!()
            // if let Definition::BuiltinValue(value) =
            // self.items[arg].definition.as_ref().unwrap() {
            //     result.push(*value);
            // } else {
            //     return None;
            // }
        }
        Some(result)
    }

    pub(super) fn begin_item(&mut self) -> ConstructId<'x> {
        let item = AnnotatedConstruct {
            cached_reduction: None,
            definition: None,
            dependencies: None,
            parent_scope: None,
            shown_from: Vec::new(),
        };
        self.items.push(item)
    }

    pub(super) fn push_con(&mut self, con: impl Construct<'x> + 'x) -> ConstructId<'x> {
        let item = self.begin_item();
        self.items[item].definition = Some(Box::new(con));
        item
    }

    pub(super) fn push_token(&mut self, token: Token<'x>) -> ConstructId<'x> {
        if let Token::Item(item) = token {
            item
        } else {
            self.push_con(CUnresolved(token))
        }
    }

    pub(super) fn get_or_push_var(&mut self, typee: VarType<'x>) -> ConstructId<'x> {
        for (id, item) in &self.items {
            todo!()
            // if let Some(Definition::Variable {
            //     typee: candidate_typee,
            //     ..
            // }) = &item.definition
            // {
            //     if &typee == candidate_typee {
            //         return id;
            //     }
            // }
        }
        self.push_var(typee)
    }

    pub(super) fn push_var(&mut self, typee: VarType<'x>) -> ConstructId<'x> {
        let var = self.vars.push(Variable { pd: PhantomData });
        let def = CVariable { var, typee };
        self.push_con(def)
    }

    pub(super) fn item_with_new_definition(
        &mut self,
        original: ConstructId<'x>,
        new_def: BoxedConstruct<'x>,
        is_fundamentally_different: bool,
    ) -> ConstructId<'x> {
        if new_def.eq(&**self.get_definition(original)) {
            return original;
        }
        let original = &self.items[original];
        let mut new_item = AnnotatedConstruct {
            definition: Some(new_def),
            parent_scope: original.parent_scope,
            dependencies: original.dependencies.clone(),
            cached_reduction: original.cached_reduction,
            shown_from: vec![],
        };
        if is_fundamentally_different {
            new_item.dependencies = None;
            new_item.cached_reduction = None;
        }
        let id = self.items.push(new_item);
        self.check(id);
        id
    }
}
