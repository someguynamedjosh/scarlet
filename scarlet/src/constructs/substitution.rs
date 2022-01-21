use super::{
    downcast_construct, variable::CVariable, BoxedConstruct, Construct, ConstructDefinition,
    ConstructId, Invariant,
};
use crate::{
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct,
    scope::Scope,
    shared::{OrderedMap, TripleBool},
};

pub type Substitutions = OrderedMap<CVariable, ConstructId>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CSubstitution(ConstructId, Substitutions);

impl CSubstitution {
    pub fn new<'x>(base: ConstructId, subs: Substitutions) -> Self {
        Self(base, subs.clone())
    }

    pub fn base(&self) -> ConstructId {
        self.0
    }

    pub fn substitutions(&self) -> &Substitutions {
        &self.1
    }

    fn substitution_justifications(&self, env: &mut Environment) -> Result<Vec<Invariant>, String> {
        let mut previous_subs = Substitutions::new();
        let mut invariants = Vec::new();
        for (target, value) in &self.1 {
            match target.can_be_assigned(*value, env, &previous_subs) {
                Ok(mut new_invs) => {
                    previous_subs.insert_no_replace(target.clone(), *value);
                    invariants.append(&mut new_invs)
                }
                Err(err) => {
                    return Err(format!(
                        "THIS EXPRESSION:\n{}\nDOES NOT SATISFY THIS REQUIREMENT:\n{}",
                        env.show(*value, *value),
                        err
                    ))
                }
            }
        }
        Ok(invariants)
    }
}

impl_any_eq_for_construct!(CSubstitution);

impl Construct for CSubstitution {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, env: &mut Environment<'x>, this: ConstructId, scope: Box<dyn Scope>) {
        if let Err(err) = self.substitution_justifications(env) {
            println!("{}", err);
            todo!("nice error");
        }
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        let mut deps = Dependencies::new();
        let base = env.get_dependencies(self.0);
        for dep in base.as_variables() {
            if let Some(rep) = self.1.get(&dep) {
                let replaced_deps = env.get_dependencies(*rep);
                for rdep in replaced_deps
                    .into_variables()
                    .skip(dep.get_substitutions().len())
                {
                    deps.push_eager(rdep);
                }
            } else {
                deps.push_eager(dep.inline_substitute(env, &self.1).unwrap());
            }
        }
        deps
    }

    fn generated_invariants<'x>(
        &self,
        this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<Invariant> {
        let mut invs = Vec::new();
        let justification = match self.substitution_justifications(env) {
            Ok(ok) => ok,
            Err(err) => {
                println!("{}", err);
                todo!("Nice error");
            }
        };
        for inv in env.generated_invariants(self.0) {
            let subbed_statement = env.substitute(inv.statement, &self.1);
            invs.push(Invariant::new(subbed_statement));
        }
        invs
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        if let Some(other) = downcast_construct::<Self>(other) {
            let mut result = env.is_def_equal(self.0, other.0);
            if self.1.len() != other.1.len() {
                result = TripleBool::False;
            }
            for (target, value) in &self.1 {
                if let Some(other_value) = other.1.get(target) {
                    result = TripleBool::and(vec![result, env.is_def_equal(*value, *other_value)]);
                } else {
                    result = TripleBool::False
                }
            }
            result
        } else {
            TripleBool::Unknown
        }
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        env.reduce(self.0);
        let subbed = env.substitute(self.0, &self.1);
        env.reduce(subbed);
        subbed.into()
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructDefinition<'x> {
        let base = self.0;
        let mut new_subs = self.1.clone();
        for (_, value) in &mut new_subs {
            let subbed = env.substitute(*value, substitutions);
            *value = subbed;
        }
        for (target, value) in substitutions {
            let mut already_present = false;
            for (existing_target, _) in &new_subs {
                if existing_target.is_same_variable_as(target) {
                    already_present = true;
                    break;
                }
            }
            if !already_present {
                new_subs.insert_no_replace(target.clone(), *value);
            }
        }
        ConstructDefinition::Resolved(Self::new(base, new_subs).dyn_clone())
    }
}
