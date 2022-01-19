use super::{
    downcast_construct, substitution::Substitutions, Construct, ConstructDefinition, ConstructId,
};
use crate::{
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct,
    scope::SPlain,
    shared::TripleBool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CIfThenElse {
    left: ConstructId,
    right: ConstructId,
    equal: ConstructId,
    unequal: ConstructId,
}

impl CIfThenElse {
    pub fn new<'x>(
        left: ConstructId,
        right: ConstructId,
        equal: ConstructId,
        unequal: ConstructId,
    ) -> Self {
        Self {
            left,
            right,
            equal,
            unequal,
        }
    }

    pub fn left(&self) -> ConstructId {
        self.left
    }

    pub fn right(&self) -> ConstructId {
        self.right
    }

    pub fn then(&self) -> ConstructId {
        self.equal
    }

    pub fn elsee(&self) -> ConstructId {
        self.unequal
    }
}

impl_any_eq_for_construct!(CIfThenElse);

impl Construct for CIfThenElse {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn generated_invariants<'x>(
        &self,
        this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<ConstructId> {
        let truee = env.get_language_item("true");
        let true_invs = env.generated_invariants(self.equal);
        let mut false_invs = env.generated_invariants(self.equal);
        let mut result = Vec::new();
        for true_inv in true_invs {
            let mut is_conditional = true;
            for (index, &false_inv) in false_invs.clone().iter().enumerate() {
                if env.is_def_equal(true_inv, false_inv) == TripleBool::True {
                    result.push(true_inv);
                    false_invs.remove(index);
                    is_conditional = false;
                }
            }
        }
        result
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        let mut deps = env.get_dependencies(self.left);
        deps.append(env.get_dependencies(self.right));
        deps.append(env.get_dependencies(self.equal));
        deps.append(env.get_dependencies(self.unequal));
        deps
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        if let Some(other) = downcast_construct::<Self>(other) {
            TripleBool::and(vec![
                env.is_def_equal(self.left, other.left),
                env.is_def_equal(self.right, other.right),
                env.is_def_equal(self.equal, other.equal),
                env.is_def_equal(self.unequal, other.unequal),
            ])
        } else {
            TripleBool::Unknown
        }
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        env.reduce(self.left);
        env.reduce(self.right);
        match env.is_def_equal(self.left, self.right) {
            TripleBool::True => {
                env.reduce(self.equal);
                self.equal.into()
            }
            TripleBool::False => {
                env.reduce(self.unequal);
                self.unequal.into()
            }
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
        let equal = env.substitute(self.equal, substitutions);
        let unequal = env.substitute(self.unequal, substitutions);
        Self::new(left, right, equal, unequal).dyn_clone()
    }
}
