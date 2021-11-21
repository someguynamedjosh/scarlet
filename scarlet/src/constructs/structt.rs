use super::{
    as_struct,
    base::{Construct, ConstructId},
    substitution::Substitutions,
    variable::{CVariable, VarType},
};
use crate::{environment::{matchh::MatchResult, Environment}, impl_any_eq_for_construct, tokens::structure::Token};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructField {
    pub name: Option<String>,
    pub value: ConstructId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CStruct(pub Vec<StructField>);

impl_any_eq_for_construct!(CStruct);

impl Construct for CStruct {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        let mut deps = Vec::new();
        for field in &self.0 {
            deps.append(&mut env.get_dependencies(field.value));
        }
        deps
    }

    fn matches_simple_var_type<'x>(&self, env: &mut Environment<'x>, pattern: &VarType) -> MatchResult {
        if let VarType::Just(pattern) = pattern {
            if let Some(pattern) = as_struct(&**env.get_construct(*pattern)) {
                if self.0.len() != pattern.0.len() {
                    return MatchResult::NoMatch;
                }
                let pattern = pattern.clone();
                let fields = self.0.iter().zip(pattern.0.iter());
                let results = fields
                    .map(|(vfield, pfield)| {
                        if vfield.name == pfield.name {
                            println!("{:?} {:?}", vfield.value, pfield.value);
                            env.construct_matches_construct(vfield.value, pfield.value)
                        } else {
                            panic!("Name mismatch!");
                            MatchResult::NoMatch
                        }
                    })
                    .collect();
                return MatchResult::and(results);
            }
        }
        MatchResult::Unknown
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>, _self_id: ConstructId) -> ConstructId {
        let mut fields = Vec::new();
        for field in &self.0 {
            fields.push(StructField {
                name: field.name.clone(),
                value: env.reduce(field.value),
            });
        }
        env.push_construct(Box::new(Self(fields)))
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let fields = self
            .0
            .iter()
            .map(|field| StructField {
                name: field.name.clone(),
                value: env.substitute(field.value, substitutions),
            })
            .collect();
        env.push_construct(Box::new(Self(fields)))
    }
}
