use super::{
    as_struct,
    base::{ConstructDefinition, ConstructId},
    substitution::Substitutions,
    Construct,
};
use crate::{
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
    ) -> Vec<ConstructId> {
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

    fn is_def_equal<'x>(&self, _env: &mut Environment<'x>, _other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> Box<dyn Construct> {
        let value = env.substitute(self.value, substitutions);
        let rest = env.substitute(self.rest, substitutions);
        Self::new(self.label.clone(), value, rest).dyn_clone()
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
    ) -> Vec<ConstructId> {
        env.generated_invariants(self.0)
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Dependencies {
        env.get_dependencies(self.0)
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
    ) -> Box<dyn Construct> {
        let subbed_base = env.substitute(self.0, substitutions);
        Self(subbed_base, self.1).dyn_clone()
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
        _invariant: ConstructId,
    ) -> bool {
        if let Some(_structt) = as_struct(&**env.get_reduced_construct_definition(self.0)) {
            false
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
        _env: &mut Environment<'x>,
        _invariant: ConstructId,
    ) -> bool {
        false
    }

    fn parent(&self) -> Option<ConstructId> {
        Some(self.0)
    }
}
