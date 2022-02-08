use super::{
    as_struct,
    base::{ConstructDefinition, ConstructId},
    substitution::{NestedSubstitutions, SubExpr, Substitutions},
    Construct, GenInvResult, Invariant,
};
use crate::{
    constructs::downcast_construct,
    environment::{
        dependencies::{DepResult, Dependencies, DependencyError},
        DefEqualResult, Environment,
    },
    impl_any_eq_for_construct,
    scope::{LookupIdentResult, LookupInvariantResult, ReverseLookupIdentResult, Scope},
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
    ) -> GenInvResult {
        Ok([
            env.generated_invariants(self.value)?,
            env.generated_invariants(self.rest)?,
        ]
        .concat())
    }

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        let mut deps = env.get_dependencies(self.value);
        deps.append(env.get_dependencies(self.rest));
        deps
    }

    fn is_def_equal<'x>(
        &self,
        env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        SubExpr(other, other_subs): SubExpr,
        recursion_limit: u32,
    ) -> DefEqualResult {
        assert_ne!(recursion_limit, 0);
        if let Some(other) = env.get_and_downcast_construct_definition::<Self>(other)? {
            let other = other.clone();
            if self.label != other.label {
                return Ok(TripleBool::False);
            }
            Ok(TripleBool::and(vec![
                env.is_def_equal(
                    SubExpr(self.value, subs),
                    SubExpr(other.value, other_subs),
                    recursion_limit - 1,
                )?,
                env.is_def_equal(
                    SubExpr(self.rest, subs),
                    SubExpr(other.rest, other_subs),
                    recursion_limit - 1,
                )?,
            ]))
        } else {
            Ok(TripleBool::Unknown)
        }
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
    ) -> GenInvResult {
        if let Some(structt) = as_struct(&**env.get_construct_definition(self.0)?) {
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

    fn get_dependencies<'x>(&self, env: &mut Environment<'x>) -> DepResult {
        let base_def = match env.get_construct_definition(self.0) {
            Ok(def) => &**def,
            Err(err) => return Dependencies::new_error(err),
        };
        if let Some(structt) = as_struct(base_def) {
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

    fn is_def_equal<'x>(
        &self,
        _env: &mut Environment<'x>,
        subs: &NestedSubstitutions,
        other: SubExpr,
        recursion_limit: u32,
    ) -> DefEqualResult {
        Ok(TripleBool::Unknown)
    }
}

#[derive(Debug, Clone)]
pub struct SField(pub ConstructId);

impl Scope for SField {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> LookupIdentResult {
        if let Some(structt) = as_struct(&**env.get_construct_definition(self.0)?) {
            let structt = structt.clone();
            Ok(if structt.label == ident {
                Some(structt.value)
            } else {
                None
            })
        } else {
            unreachable!()
        }
    }

    fn local_reverse_lookup_ident<'x>(
        &self,
        env: &mut Environment<'x>,
        value: ConstructId,
    ) -> ReverseLookupIdentResult {
        if let Some(structt) = as_struct(&**env.get_construct_definition(self.0)?) {
            let structt = structt.clone();
            Ok(if structt.value == value {
                Some(structt.label.clone())
            } else {
                None
            })
        } else {
            unreachable!()
        }
    }

    fn local_lookup_invariant<'x>(
        &self,
        env: &mut Environment<'x>,
        invariant: ConstructId,
        limit: u32,
    ) -> LookupInvariantResult {
        if let Some(structt) = as_struct(&**env.get_construct_definition(self.0)?) {
            let structt = structt.clone();
            for maybe_match in env.generated_invariants(structt.value)? {
                if env.is_def_equal(
                    SubExpr(invariant, &Default::default()),
                    SubExpr(maybe_match.statement, &Default::default()),
                    limit,
                )? == TripleBool::True
                {
                    return Ok(Some(maybe_match));
                }
            }
            Ok(None)
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
) -> LookupIdentResult {
    Ok(if inn.label == ident {
        Some(inn.value)
    } else if let Some(rest) = as_struct(&**env.get_construct_definition(inn.rest)?) {
        let rest = rest.clone();
        lookup_ident_in(env, ident, &rest)?
    } else {
        None
    })
}

fn reverse_lookup_ident_in<'x>(
    env: &mut Environment<'x>,
    value: ConstructId,
    inn: &CPopulatedStruct,
) -> ReverseLookupIdentResult {
    Ok(if inn.value == value {
        Some(inn.label.clone())
    } else if let Some(rest) = as_struct(&**env.get_construct_definition(inn.rest)?) {
        let rest = rest.clone();
        reverse_lookup_ident_in(env, value, &rest)?
    } else {
        None
    })
}

fn lookup_invariant_in<'x>(
    env: &mut Environment<'x>,
    invariant: ConstructId,
    inn: &CPopulatedStruct,
    limit: u32,
) -> LookupInvariantResult {
    if let Some(rest) = as_struct(&**env.get_construct_definition(inn.rest)?) {
        let rest = rest.clone();
        for maybe_match in env.generated_invariants(rest.value)? {
            if env.is_def_equal(
                SubExpr(invariant, &Default::default()),
                SubExpr(maybe_match.statement, &Default::default()),
                limit,
            )? == TripleBool::True
            {
                return Ok(Some(maybe_match));
            }
        }
        lookup_invariant_in(env, invariant, &rest, limit)
    } else {
        Ok(None)
    }
}

impl Scope for SFieldAndRest {
    fn dyn_clone(&self) -> Box<dyn Scope> {
        Box::new(self.clone())
    }

    fn local_lookup_ident<'x>(&self, env: &mut Environment<'x>, ident: &str) -> LookupIdentResult {
        if let Some(structt) = as_struct(&**env.get_construct_definition(self.0)?) {
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
    ) -> ReverseLookupIdentResult {
        if let Some(structt) = as_struct(&**env.get_construct_definition(self.0)?) {
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
        limit: u32,
    ) -> LookupInvariantResult {
        if let Some(structt) = as_struct(&**env.get_construct_definition(self.0)?) {
            let structt = structt.clone();
            lookup_invariant_in(env, invariant, &structt, limit)
        } else {
            unreachable!()
        }
    }

    fn parent(&self) -> Option<ConstructId> {
        Some(self.0)
    }
}
