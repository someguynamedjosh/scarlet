use super::{
    as_struct,
    base::{Construct, ConstructId},
    downcast_construct,
    substitution::Substitutions,
    variable::CVariable,
    ConstructDefinition,
};
use crate::{
    environment::Environment, impl_any_eq_for_construct, scope::Scope, shared::TripleBool,
};

pub fn struct_from_unnamed_fields<'x>(
    env: &mut Environment<'x>,
    mut fields: Vec<ConstructId>,
) -> ConstructId {
    if fields.is_empty() {
        env.push_construct(Box::new(CEmptyStruct))
    } else {
        let first_field = fields.remove(0);
        let rest = struct_from_unnamed_fields(env, fields);
        let con = CPopulatedStruct {
            label: String::new(),
            value: first_field,
            rest,
        };
        env.push_construct(Box::new(con))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CEmptyStruct;

impl_any_eq_for_construct!(CEmptyStruct);

impl Construct for CEmptyStruct {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        vec![]
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        if let Some(_) = downcast_construct::<Self>(other) {
            TripleBool::True
        } else {
            TripleBool::Unknown
        }
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        env.push_construct(Box::new(Self))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CPopulatedStruct {
    pub label: String,
    pub value: ConstructId,
    pub rest: ConstructId,
}

impl_any_eq_for_construct!(CPopulatedStruct);

impl Construct for CPopulatedStruct {
    fn dyn_clone(&self) -> Box<dyn Construct> {
        Box::new(self.clone())
    }

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        [
            env.get_dependencies(self.rest),
            env.get_dependencies(self.value),
        ]
        .concat()
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn substitute<'x>(
        &self,
        env: &mut Environment<'x>,
        substitutions: &Substitutions,
    ) -> ConstructId {
        let subbed = Self {
            label: self.label.clone(),
            value: env.substitute(self.value, substitutions),
            rest: env.substitute(self.rest, substitutions),
        };
        env.push_construct(Box::new(subbed))
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

    fn check<'x>(&self, _env: &mut Environment<'x>) {}

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> Vec<CVariable> {
        env.get_dependencies(self.0)
    }

    fn is_def_equal<'x>(&self, env: &mut Environment<'x>, other: &dyn Construct) -> TripleBool {
        TripleBool::Unknown
    }

    fn reduce<'x>(&self, env: &mut Environment<'x>) -> ConstructDefinition<'x> {
        env.reduce(self.0);
        if let Some(structt) = as_struct(&**env.get_construct(self.0)) {
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
    ) -> ConstructId {
        let subbed_base = env.substitute(self.0, substitutions);
        let subbed = Self(subbed_base, self.1);
        env.push_construct(Box::new(subbed))
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
    } else if let Some(rest) = as_struct(&**env.get_construct(inn.rest)) {
        let rest = rest.clone();
        lookup_ident_in(env, ident, &rest)
    } else {
        None
    }
}

impl Scope for SFieldAndRest {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> Option<ConstructId> {
        if let Some(structt) = as_struct(&**env.get_construct(self.0)) {
            let structt = structt.clone();
            lookup_ident_in(env, ident, &structt)
        } else {
            unreachable!()
        }
    }
}
