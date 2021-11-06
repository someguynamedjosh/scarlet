use super::structure::{Substitutions, UnresolvedSubstitution};
use crate::{
    shared::OrderedSet,
    stage2::{
        matchh::MatchResult,
        structure::{Definition, Environment, ItemId, VariableInfo},
    },
};

impl<'x> Environment<'x> {
    pub(super) fn resolve_custom(&mut self, item: ItemId<'x>) {
        if let Definition::CustomItem { name, contents } = self.get_definition(item) {
            let new_def = match *name {
                "other" => self.items[contents[0]].definition.clone().unwrap(),
                "substitution" => {
                    let mut contents = contents.clone();
                    let base = contents.remove(0);
                    let mut subs = contents;
                    todo!()
                    // let new_subs = self.resolve_targets_in_sub(base, &mut
                    // subs);
                }
                _ => todo!("Nice error, unrecognized custom item {}", name),
            };
            self.items[item].definition = Some(new_def);
        }
    }

    fn resolve_targets_in_sub(
        &mut self,
        base: ItemId<'x>,
        subs: &mut [UnresolvedSubstitution<'x>],
    ) -> Substitutions<'x> {
        let mut new_subs = Substitutions::new();
        let mut deps = self.get_deps(base);
        for sub in &mut *subs {
            if let Some(possible_meaning) = sub.target_meaning {
                let additional_subs = self.resolve_named_target(
                    possible_meaning,
                    sub.target_name,
                    base,
                    sub.value,
                    &mut deps,
                    &new_subs,
                );
                new_subs = new_subs.union(additional_subs);
            }
        }
        for sub in &mut *subs {
            if sub.target_meaning.is_none() {
                let additions = self.resolve_anonymous_target(&mut deps, &new_subs, sub.value);
                new_subs = new_subs.union(additions);
            }
        }
        new_subs
    }

    fn resolve_named_target(
        &mut self,
        possible_meaning: ItemId<'x>,
        name: Option<&str>,
        base: ItemId<'x>,
        value: ItemId<'x>,
        deps: &mut OrderedSet<VariableInfo<'x>>,
        new_subs: &Substitutions<'x>,
    ) -> Substitutions<'x> {
        let mut resolved_target = possible_meaning;
        if let Some(name) = name {
            if let Some(value) = self.items[base].scope.get(name) {
                resolved_target = *value;
            }
        }
        let resolved_target = self.substitute(resolved_target, new_subs).unwrap();
        match self.matches(value, resolved_target) {
            MatchResult::Match(subs) => {
                for &(target, _) in &subs {
                    for (entry, _) in &*deps {
                        if entry.var == target {
                            let entry = *entry;
                            deps.remove(&entry);
                            break;
                        }
                    }
                }
                subs
            }
            MatchResult::NoMatch => {
                todo!("Nice error, value will not match what it's assigned to.")
            }
            MatchResult::Unknown => {
                todo!("Nice error, value might not match what it's assigned to.")
            }
        }
    }

    fn resolve_anonymous_target(
        &mut self,
        deps: &mut OrderedSet<VariableInfo<'x>>,
        previous_subs: &Substitutions<'x>,
        value: ItemId<'x>,
    ) -> Substitutions<'x> {
        for (dep, _) in &*deps {
            let _dep = *dep;
            let subbed_dep = self.substitute(dep.var_item, previous_subs).unwrap();
            let subbed_dep = self.reduce(subbed_dep);
            let value = self.reduce(value);
            let result = self.matches(value, subbed_dep);
            if let MatchResult::Match(matched_subs) = result {
                for (matched_dep, _) in &matched_subs {
                    for (dep, _) in &*deps {
                        if dep.var == *matched_dep {
                            let dep = *dep;
                            deps.remove(&dep);
                            break;
                        }
                    }
                }
                return matched_subs;
            }
        }
        println!("{:#?}", self);
        todo!(
            "Nice error, the argument {:?} cannot be assigned to any of {:?}",
            value,
            deps
        )
    }
}
