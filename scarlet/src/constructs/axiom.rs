use itertools::Itertools;
use maplit::hashset;

use super::{base::Construct, ConstructId, GenInvResult};
use crate::{
    constructs::Invariant,
    environment::{
        def_equal::{DefEqualResult, IsDefEqual},
        dependencies::DepResult,
        sub_expr::{NestedSubstitutions, SubExpr},
        Environment,
    },
    impl_any_eq_for_construct,
    shared::TripleBool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CAxiom {
    statement: ConstructId,
}

impl CAxiom {
    fn new(env: &mut Environment, statement: &str) -> Self {
        Self {
            statement: env.get_language_item(statement),
        }
    }

    pub fn from_name(env: &mut Environment, name: &str) -> Self {
        Self::new(env, &format!("{}_statement", name))
    }

    pub fn get_statement(&self, env: &mut Environment) -> &'static str {
        for limit in 0..32 {
            for lang_item_name in env.language_item_names().copied().collect_vec() {
                let lang_item = env.get_language_item(lang_item_name);
                if env.is_def_equal(
                    SubExpr(self.statement, &Default::default()),
                    SubExpr(lang_item, &Default::default()),
                    limit,
                ) == Ok(IsDefEqual::Yes)
                {
                    return lang_item_name;
                }
            }
        }
        panic!("{:?} is not an axiom statement", self.statement)
    }
}

impl_any_eq_for_construct!(CAxiom);

impl Construct for CAxiom {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn generated_invariants<'x>(
        &self,
        _this: ConstructId,
        _env: &mut Environment<'x>,
    ) -> GenInvResult {
        vec![Invariant::new(self.statement, hashset![])]
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        env.get_dependencies(self.statement)
    }

    fn symm_is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        other: SubExpr,
        recursion_limit: u32,
    ) -> DefEqualResult {
        assert_ne!(recursion_limit, 0);
        let other_subs = other.1;
        if let Some(other) = env.get_and_downcast_construct_definition::<Self>(other.0)? {
            let other = other.clone();
            env.is_def_equal(
                SubExpr(self.statement, subs),
                SubExpr(other.statement, other_subs),
                recursion_limit - 1,
            )
        } else {
            Ok(IsDefEqual::Unknowable)
        }
    }
}
