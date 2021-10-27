use std::collections::HashSet;

use super::structure::{
    Environment, ItemId, StructField, Substitution, VariableId, VariableItemIds,
};
use crate::{
    shared::OrderedSet,
    stage1::structure::TokenTree,
    stage2::{
        matchh::MatchResult,
        structure::{After, BuiltinOperation, BuiltinValue, Definition, Target},
    },
};

impl<'x> Environment<'x> {
    pub fn resolve_targets(&mut self) {
        let mut next_id = self.items.first();
        while let Some(id) = next_id {
            self.resolve_targets_in_item(id);
            next_id = self.items.next(id);
        }
    }

    fn resolve_targets_in_item(&mut self, id: ItemId<'x>) {
        if let Definition::Substitute(base, subs) = self.definition_of(id) {
            let base = *base;
            let mut subs = subs.clone();
            self.resolve_targets_in_sub(base, &mut subs);
            self.items[id].definition = Some(Definition::Substitute(base, subs));
        }
    }

    fn resolve_targets_in_sub(&mut self, base: ItemId<'x>, subs: &mut [Substitution<'x>]) {
        let mut deps = self.get_deps(base);
        for sub in &mut *subs {
            if let &Target::Unresolved {
                name,
                possible_meaning,
            } = &sub.target
            {
                self.resolve_named_target(possible_meaning, name, base, &mut deps, sub);
            }
        }
        for sub in &mut *subs {
            if let Target::UnresolvedAnonymous = &sub.target {
                self.resolve_anonymous_target(&mut deps, sub);
            }
        }
    }

    fn resolve_named_target(
        &mut self,
        possible_meaning: ItemId<'x>,
        name: Option<&str>,
        base: ItemId<'x>,
        deps: &mut OrderedSet<VariableItemIds<'x>>,
        sub: &mut Substitution<'x>,
    ) {
        let mut resolved = possible_meaning;
        if let Some(name) = name {
            if let Some(value) = self.items[base].scope.get(name) {
                resolved = *value;
            }
        }
        for (dep, _) in self.get_deps(resolved) {
            deps.remove(&dep);
        }
        sub.target = Target::ResolvedItem(resolved);
    }

    fn resolve_anonymous_target(
        &mut self,
        deps: &mut OrderedSet<VariableItemIds<'x>>,
        sub: &mut Substitution<'x>,
    ) {
        let mut success = false;
        for (dep, _) in &*deps {
            let dep = *dep;
            let matches = dep.matches;
            let value = self.reduce(sub.value);
            let result = self.matches(value, matches) ;
            println!("{:?} matches {:?}? {:?}", value, matches, result);
            if let MatchResult::Match(..) = result {
                success = true;
                deps.remove(&dep);
                sub.target = Target::ResolvedItem(dep.var_item);
                break;
            }
        }
        if !success {
            // println!("{:#?}", self);
            todo!(
                "Nice error, the argument {:?} cannot be assigned to any of {:?}",
                sub.value,
                deps
            );
        }
    }
}
