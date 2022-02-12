use std::collections::HashSet;

use super::{BoxedResolvable, Resolvable, ResolveError, ResolveResult};
use crate::{
    constructs::{
        structt::{AtomicStructMember, CAtomicStructMember, CPopulatedStruct},
        substitution::{CSubstitution, Substitutions},
        variable::CVariable,
        Construct, ConstructDefinition, ConstructId,
    },
    environment::{Environment, UnresolvedConstructError},
    scope::Scope,
    shared::OrderedMap,
};

#[derive(Clone, Debug)]
pub struct RNamedMember<'x> {
    pub base: ConstructId,
    pub member_name: &'x str,
}

fn find_member(
    env: &mut Environment,
    inn: ConstructId,
    name: &str,
) -> Result<Option<u32>, UnresolvedConstructError> {
    if let Some(cstruct) = env.get_and_downcast_construct_definition::<CPopulatedStruct>(inn)? {
        if cstruct.get_label() == name {
            Ok(Some(0))
        } else {
            let rest = cstruct.get_rest();
            Ok(find_member(env, rest, name)?.map(|x| x + 1))
        }
    } else {
        Ok(None)
    }
}

impl<'x> Resolvable<'x> for RNamedMember<'x> {
    fn dyn_clone(&self) -> BoxedResolvable<'x> {
        Box::new(self.clone())
    }

    fn resolve(
        &self,
        env: &mut Environment<'x>,
        scope: Box<dyn Scope>,
        _limit: u32,
    ) -> ResolveResult<'x> {
        let access_depth = find_member(env, self.base, self.member_name)?;
        let access_depth = if let Some(ad) = access_depth {
            ad
        } else {
            todo!(
                "Nice error, failed to find a member named {}.",
                self.member_name
            );
        };
        let mut base = self.base;
        for _ in 0..access_depth {
            base = env.push_construct(
                CAtomicStructMember(base, AtomicStructMember::Rest),
                scope.dyn_clone(),
            );
        }
        let def = CAtomicStructMember(base, AtomicStructMember::Value);
        Ok(ConstructDefinition::Resolved(def.dyn_clone()))
    }
}