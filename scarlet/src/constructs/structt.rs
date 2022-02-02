use super::{
    as_struct,
    base::{ConstructDefinition, ConstructId},
    substitution::Substitutions,
    Construct, Invariant,
};
use crate::{
    constructs::downcast_construct,
    environment::{dependencies::Dependencies, Environment},
    impl_any_eq_for_construct,
    scope::Scope,
    shared::TripleBool,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CPopulatedStruct {
    label: String,
    value: ConstructId,
    rest: ConstructId,
}

impl CPopulatedStruct {
    pub fn new<'x>(label: String, value: ConstructId, rest: ConstructId) -> Self {
        Self { label, value, rest }
    }

    pub fn get_label(&self) -> &str {
        &self.label[..]
    }

    pub fn get_value(&self) -> ConstructId {
        self.value
    }

    pub fn get_rest(&self) -> ConstructId {
        self.rest
    }
}

impl_any_eq_for_construct!(CPopulatedStruct);

impl Construct for CPopulatedStruct {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn generated_invariants<'x>(
        &self,
        _this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<Invariant> {
        [
            env.generated_invariants(self.value),
            env.generated_invariants(self.rest),
        ]
        .concat()
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        let mut deps = env.get_dependencies(self.rest);
        deps.append(env.get_dependencies(self.value));
        deps
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        if let Some(other) = downcast_construct::<Self>(other) {
            if self.label != other.label {
                return TripleBool::False;
            }
            TripleBool::and(vec![
                env.is_def_equal(self.value, other.value),
                env.is_def_equal(self.rest, other.rest),
            ])
        } else {
            TripleBool::Unknown
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructDefinition<'x> {
        let value = env.substitute(self.value, substitutions);
        let rest = env.substitute(self.rest, substitutions);
        ConstructDefinition::Resolved(Self::new(self.label.clone(), value, rest).dyn_clone())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtomicStructMember {
    Label,
    Value,
    Rest,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CAtomicStructMember(pub ConstructId, pub AtomicStructMember);

impl_any_eq_for_construct!(CAtomicStructMember);

impl Construct for CAtomicStructMember {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn generated_invariants<'x>(
        &self,
        _this: ConstructId,
        env: &mut Environment<'x>,
    ) -> Vec<Invariant> {
        if let Some(structt) = as_struct(&**env.get_reduced_construct_definition(self.0)) {
            let structt = structt.clone();
            match self.1 {
                AtomicStructMember::Label => todo!(),
                AtomicStructMember::Value => env.generated_invariants(structt.value),
                AtomicStructMember::Rest => env.generated_invariants(structt.rest),
            }
        } else {
            env.generated_invariants(self.0)
        }
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        if let Some(structt) = as_struct(&**env.get_reduced_construct_definition(self.0)) {
            let structt = structt.clone();
            match self.1 {
                AtomicStructMember::Label => todo!(),
                AtomicStructMember::Value => env.get_dependencies(structt.value),
                AtomicStructMember::Rest => env.get_dependencies(structt.rest),
            }
        } else {
            env.get_dependencies(self.0)
        }
    }

    fn is_def_equal<'x>(&self, _env: &mut Environment<'x>, _other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        if let Some(structt) = as_struct(&**env.get_reduced_construct_definition(self.0)) {
            match self.1 {
                AtomicStructMember::Label => todo!(),
                AtomicStructMember::Value => structt.value.into(),
                AtomicStructMember::Rest => structt.rest.into(),
            }
        } else {
            self.dyn_clone().into()
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructDefinition<'x> {
        let subbed_base = env.substitute(self.0, substitutions);
        ConstructDefinition::Resolved(Self(subbed_base, self.1).dyn_clone())
    }
}

#[derive(Debug, Clone)]
pub struct SField(pub ConstructId);

impl Scope for SField {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident<'x>(
        &self,
        env: &mut Environment<'x>,
        ident: &str,
    ) -> Option<ConstructId> {
        if let Some(structt) = as_struct(&**env.get_reduced_construct_definition(self.0)) {
            let structt = structt.clone();
            if structt.label == ident {
                Some(structt.value)
            } else {
                None
            }
        } else {
            unreachable!()
        }
    }

    fn local_reverse_lookup_ident<'x>(
        &self,
        env: &mut Environment<'x>,
        value: ConstructId,
    ) -> Option<String> {
        if let Some(structt) = as_struct(&**env.get_reduced_construct_definition(self.0)) {
            let structt = structt.clone();
            if structt.value == value {
                Some(structt.label.clone())
            } else {
                None
            }
        } else {
            unreachable!()
        }
    }

    fn local_lookup_invariant<'x>(
        &self,
        env: &mut Environment<'x>,
        invariant: ConstructId,
    ) -> Option<Invariant> {
        if let Some(structt) = as_struct(&**env.get_reduced_construct_definition(self.0)) {
            let structt = structt.clone();
            for maybe_match in env.generated_invariants(structt.value) {
                if env.is_def_equal(invariant, maybe_match.statement) == TripleBool::True {
                    return Some(maybe_match);
                }
            }
            None
        } else {
            unreachable!()
        }
    }

    fn parent(&self) -> Option<ConstructId> {
        Some(self.0)
    }
}

#[derive(Debug, Clone)]
pub struct SFieldAndRest(pub ConstructId);

fn lookup_ident_in<'x>(
    env: &mut Environment<'x>,
    ident: &str,
    inn: &CPopulatedStruct,
) -> Option<ConstructId> {
    if inn.label == ident {
        Some(inn.value)
    } else if let Some(rest) = as_struct(&**env.get_reduced_construct_definition(inn.rest)) {
        let rest = rest.clone();
        lookup_ident_in(env, ident, &rest)
    } else {
        None
    }
}

fn reverse_lookup_ident_in<'x>(
    env: &mut Environment<'x>,
    value: ConstructId,
    inn: &CPopulatedStruct,
) -> Option<String> {
    if inn.value == value {
        Some(inn.label.clone())
    } else if let Some(rest) = as_struct(&**env.get_reduced_construct_definition(inn.rest)) {
        let rest = rest.clone();
        reverse_lookup_ident_in(env, value, &rest)
    } else {
        None
    }
}

fn lookup_invariant_in<'x>(
    env: &mut Environment<'x>,
    invariant: ConstructId,
    inn: &CPopulatedStruct,
) -> Option<Invariant> {
    if let Some(rest) = as_struct(&**env.get_reduced_construct_definition(inn.rest)) {
        let rest = rest.clone();
        for maybe_match in env.generated_invariants(rest.value) {
            if env.is_def_equal(invariant, maybe_match.statement) == TripleBool::True {
                return Some(maybe_match);
            }
        }
        lookup_invariant_in(env, invariant, &rest)
    } else {
        None
    }
}

impl Scope for SFieldAndRest {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident<'x>(
        &self,
        env: &mut Environment<'x>,
        ident: &str,
    ) -> Option<ConstructId> {
        if let Some(structt) = as_struct(&**env.get_reduced_construct_definition(self.0)) {
            let structt = structt.clone();
            lookup_ident_in(env, ident, &structt)
        } else {
            unreachable!()
        }
    }

    fn local_reverse_lookup_ident<'a, 'x>(
        &self,
        env: &'a mut Environment<'x>,
        value: ConstructId,
    ) -> Option<String> {
        if let Some(structt) = as_struct(&**env.get_reduced_construct_definition(self.0)) {
            let structt = structt.clone();
            reverse_lookup_ident_in(env, value, &structt)
        } else {
            unreachable!()
        }
    }

    fn local_lookup_invariant<'x>(
        &self,
        env: &mut Environment<'x>,
        invariant: ConstructId,
    ) -> Option<Invariant> {
        if let Some(structt) = as_struct(&**env.get_reduced_construct_definition(self.0)) {
            let structt = structt.clone();
            lookup_invariant_in(env, invariant, &structt)
        } else {
            unreachable!()
        }
    }

    fn parent(&self) -> Option<ConstructId> {
        Some(self.0)
    }
}
