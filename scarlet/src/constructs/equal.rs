use itertools::Itertools;

use super::{
    if_then_else::CIfThenElse, substitution::Substitutions, variable::CVariable, Construct,
    ConstructDefinition, ConstructId,
};
use crate::{
    environment::Environment, impl_any_eq_for_construct, scope::SPlain, shared::TripleBool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CEqual {
    left: ConstructId,
    right: ConstructId,
}

impl CEqual {
    pub fn new<'x>(
        env: &mut Environment<'x>,
        left: ConstructId,
        right: ConstructId,
    ) -> ConstructId {
        let con = env.push_construct(Self { left, right });
        env.set_scope(left, &SPlain(con));
        env.set_scope(right, &SPlain(con));
        con
    }
}

impl_any_eq_for_construct!(CEqual);

impl Construct for CEqual {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        [
            env.get_dependencies(self.left),
            env.get_dependencies(self.right),
        ]
        .concat()
    }

    fn generated_invariants<'x>(
        &self,
        this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<ConstructId> {
        let truee = env.get_builtin_item("true");
        let falsee = env.get_builtin_item("false");
        let is_true = Self::new(env, this, truee);
        let is_false = Self::new(env, this, falsee);
        let is_bool = CIfThenElse::new(env, is_true, truee, is_false);

        env.set_scope(is_bool, &SPlain(this));

        env.reduce(is_true);
        env.reduce(is_false);
        env.reduce(is_bool);

        [
            vec![is_bool],
            env.generated_invariants(self.left),
            env.generated_invariants(self.right),
        ]
        .concat()
    }

    fn is_def_equal<'x>(&self, _env: &mut Environment<'x>, _other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        match env.is_def_equal(self.left, self.right) {
            TripleBool::True => env.get_builtin_item("true").into(),
            TripleBool::False => env.get_builtin_item("false").into(),
            TripleBool::Unknown => self.dyn_clone().into(),
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let left = env.substitute(self.left, substitutions);
        let right = env.substitute(self.right, substitutions);
        Self::new(env, left, right)
    }
}
