use maplit::hashset;

use super::{
    downcast_construct,
    substitution::{NestedSubstitutions, SubExpr, Substitutions},
    Construct, ConstructDefinition, ConstructId, Invariant,
};
use crate::{
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct,
    scope::SPlain,
    shared::TripleBool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CDecision {
    left: ConstructId,
    right: ConstructId,
    equal: ConstructId,
    unequal: ConstructId,
}

impl CDecision {
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

    pub fn equal(&self) -> ConstructId {
        self.equal
    }

    pub fn unequal(&self) -> ConstructId {
        self.unequal
    }
}

impl_any_eq_for_construct!(CDecision);

impl Construct for CDecision {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn generated_invariants<'x>(
        &self,
        this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<Invariant> {
        let true_invs = env.generated_invariants(self.equal);
        let mut false_invs = env.generated_invariants(self.equal);
        let mut result = Vec::new();
        for true_inv in true_invs {
            for (index, false_inv) in false_invs.clone().into_iter().enumerate() {
                if env.is_def_equal(
                    SubExpr(true_inv.statement, &Default::default()),
                    SubExpr(false_inv.statement, &Default::default()),
                ) == TripleBool::True
                {
                    let mut deps = true_inv.dependencies;
                    deps.insert(this);
                    result.push(Invariant::new(true_inv.statement, deps));
                    false_invs.remove(index);
                    break;
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

    fn is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        SubExpr(other, other_subs): SubExpr,
    ) -> TripleBool {
        let base = if let Some(other) = env.get_and_downcast_construct_definition::<Self>(other) {
            let other = other.clone();
            TripleBool::and(vec![
                env.is_def_equal(SubExpr(self.left, subs), SubExpr(other.left, other_subs)),
                env.is_def_equal(SubExpr(self.right, subs), SubExpr(other.right, other_subs)),
                env.is_def_equal(SubExpr(self.equal, subs), SubExpr(other.equal, other_subs)),
                env.is_def_equal(
                    SubExpr(self.unequal, subs),
                    SubExpr(other.unequal, other_subs),
                ),
            ])
        } else {
            TripleBool::Unknown
        };
        let other = env.get_construct_definition(other).dyn_clone();
        TripleBool::or(vec![
            base,
            match env.is_def_equal(SubExpr(self.left, subs), SubExpr(self.right, other_subs)) {
                TripleBool::True => other.is_def_equal(env, other_subs, SubExpr(self.equal, subs)),
                TripleBool::False => {
                    other.is_def_equal(env, other_subs, SubExpr(self.unequal, subs))
                }
                TripleBool::Unknown => TripleBool::False,
            },
        ])
    }
}
