use super::{
    base::Construct, downcast_construct, substitution::Substitutions, BoxedConstruct,
    ConstructDefinition, ConstructId,
};
use crate::{
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct,
    shared::{Id, Pool, TripleBool},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CAxiom {
    statement: ConstructId,
    requires: Vec<ConstructId>,
}

impl CAxiom {
    fn new(env: &mut Environment, statement: &str, requires: &[&str]) -> Self {
        Self {
            statement: env.get_language_item(statement),
            requires: requires.iter().map(|i| env.get_language_item(*i)).collect(),
        }
    }

    pub fn t_just(env: &mut Environment) -> Self {
        Self::new(env, "t_just_statement", &["t_just_statement"])
    }

    pub fn t_trivial(env: &mut Environment) -> Self {
        Self::new(env, "t_trivial_statement", &[])
    }

    pub fn from_name(env: &mut Environment, name: &str) -> Option<Self> {
        Some(match name {
            "t_just" => Self::t_just(env),
            "t_trivial" => Self::t_trivial(env),
            _ => return None,
        })
    }
}

impl_any_eq_for_construct!(CAxiom);

impl Construct for CAxiom {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn generated_invariants<'x>(
        &self,
        this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<ConstructId> {
        vec![self.statement]
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        let mut base = Dependencies::new();
        for &r in &self.requires {
            base.append(env.get_dependencies(r));
        }
        base.append(env.get_dependencies(self.statement));
        base
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        env.get_language_item("void").into()
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        if let Some(other) = downcast_construct::<Self>(other) {
            TripleBool::True
        } else {
            TripleBool::Unknown
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> BoxedConstruct {
        Box::new(Self {
            statement: env.substitute(self.statement, substitutions),
            requires: self
                .requires
                .iter()
                .map(|r| env.substitute(*r, substitutions))
                .collect(),
        })
    }
}
