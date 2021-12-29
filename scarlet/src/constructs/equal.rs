use itertools::Itertools;

use super::{
    downcast_construct, if_then_else::CIfThenElse, substitution::Substitutions,
    variable::CVariable, Construct, ConstructDefinition, ConstructId,
};
use crate::{
    environment::Environment,
    impl_any_eq_for_construct,
    scope::{SPlain, Scope},
    shared::TripleBool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CEqual {
    left: ConstructId,
    right: ConstructId,
}

impl CEqual {
    pub fn new<'x>(left: ConstructId, right: ConstructId) -> Self {
        Self { left, right }
    }

    pub fn left(&self) -> ConstructId {
        self.left
    }

    pub fn right(&self) -> ConstructId {
        self.right
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
        let truee = env.get_language_item("true");
        let falsee = env.get_language_item("false");
        let is_true = this;
        let is_false = env.push_construct(Self::new(this, falsee), Box::new(SPlain(this)));
        let is_bool = env.push_construct(
            CIfThenElse::new(is_true, truee, is_false),
            Box::new(SPlain(this)),
        );

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

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        if let Some(other) = downcast_construct::<Self>(other) {
            TripleBool::and(vec![
                env.is_def_equal(self.left, other.left),
                env.is_def_equal(self.right, other.right),
            ])
        } else {
            TripleBool::Unknown
        }
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        match env.is_def_equal(self.left, self.right) {
            TripleBool::True => env.get_language_item("true").into(),
            TripleBool::False => env.get_language_item("false").into(),
            TripleBool::Unknown => self.dyn_clone().into(),
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> Box<dyn Construct> {
        let left = env.substitute(self.left, substitutions);
        let right = env.substitute(self.right, substitutions);
        Self::new(left, right).dyn_clone()
    }
}
