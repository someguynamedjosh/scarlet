use maplit::hashset;

use super::{
    base::Construct, downcast_construct, substitution::Substitutions, GenInvResult, ItemId,
};
use crate::{
    environment::{
        dependencies::DepResult,
        discover_equality::{DeqResult, DeqSide, Equal},
        invariants::InvariantSet,
        Environment,
    },
    impl_any_eq_for_construct,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CAxiom {
    statement: ItemId,
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
            for lang_item_name in env.language_item_names() {
                let lang_item = env.get_language_item(lang_item_name);
                if env.discover_equal(self.statement, lang_item, limit) == Ok(Equal::yes()) {
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

    fn generated_invariants<'x>(&self, _this: ItemId, env: &mut Environment<'x>) -> GenInvResult {
        env.push_invariant_set(InvariantSet::new(vec![self.statement], vec![], hashset![]))
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        env.get_dependencies(self.statement)
    }

    fn discover_equality<'x>(
        &self,
        env: &mut Environment<'x>,
        self_subs: Vec<&Substitutions>,
        other_id: ItemId,
        other: &dyn Construct,
        other_subs: Vec<&Substitutions>,
        limit: u32,
    ) -> DeqResult {
        if let Some(other) = downcast_construct::<Self>(other) {
            let other = other.clone();
            env.discover_equal_with_subs(
                self.statement,
                self_subs,
                other.statement,
                other_subs,
                limit,
            )
        } else {
            Ok(Equal::Unknown)
        }
    }
}
