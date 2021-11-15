use itertools::Itertools;

use super::construct::{
    constructs::{CBuiltinValue, CSubstitute, Substitutions},
    BoxedConstruct, Construct,
};
use crate::{
    shared::OrderedSet,
    stage2::{
        construct::constructs::CUnresolved,
        matchh::MatchResult,
        structure::{BuiltinValue, ConstructId, Environment, Token, VariableInfo},
        transform::{self, ApplyContext},
    },
};

impl<'x> Environment<'x> {
    pub(super) fn resolve(&mut self, item: ConstructId<'x>) -> ConstructId<'x> {
        let parent_scope = self.items[item].parent_scope;
        todo!()
    }

    fn resolve_plain_token(&mut self, item: ConstructId<'x>, plain: &str) -> BoxedConstruct<'x> {
        if let Ok(int) = plain.parse::<u32>() {
            Box::new(CBuiltinValue(BuiltinValue::_32U(int)))
        } else if plain == "true" {
            Box::new(CBuiltinValue(BuiltinValue::Bool(true)))
        } else if plain == "false" {
            Box::new(CBuiltinValue(BuiltinValue::Bool(false)))
        } else {
            let mut maybe_scope = self.items[item].parent_scope;
            while let Some(scope) = maybe_scope {
                todo!()
                // if let Some(result) = self.get_member(scope, plain) {
                //     return Box::new(CUnresolved(Token::Item(result)));
                // }
                // maybe_scope = self.items[scope].parent_scope;
            }
            println!("{:#?}\n{:?}", self, item);
            todo!("Nice error, bad ident {}", plain)
        }
    }

    fn resolve_targets_in_sub(
        &mut self,
        base: ConstructId<'x>,
        subs: &mut [Token<'x>],
        parent_scope: Option<ConstructId<'x>>,
    ) -> Substitutions<'x> {
        let mut new_subs = super::construct::constructs::Substitutions::new();
        let mut deps = self.get_deps(base);
        for sub in &mut *subs {
            if let Token::Stream {
                label: "target",
                contents,
            } = sub
            {
                let (target, value) = contents.into_iter().collect_tuple().unwrap();
                let possible_meaning = self.push_token(target.clone());
                self.items[possible_meaning].parent_scope = parent_scope;
                let value = self.push_token(value.clone());
                self.items[value].parent_scope = parent_scope;
                let additional_subs = self.resolve_named_target(
                    possible_meaning,
                    None,
                    base,
                    value,
                    &mut deps,
                    &new_subs,
                );
                new_subs = new_subs.union(additional_subs);
            }
        }
        for sub in &mut *subs {
            if let Token::Stream {
                label: "target", ..
            } = sub
            {
            } else {
                let value = self.push_token(sub.clone());
                self.items[value].parent_scope = parent_scope;
                let additions = self.resolve_anonymous_target(&mut deps, &new_subs, value);
                new_subs = new_subs.union(additions);
            }
        }
        new_subs
    }

    fn resolve_named_target(
        &mut self,
        possible_meaning: ConstructId<'x>,
        _name: Option<&str>,
        _base: ConstructId<'x>,
        value: ConstructId<'x>,
        deps: &mut OrderedSet<VariableInfo<'x>>,
        new_subs: &Substitutions<'x>,
    ) -> Substitutions<'x> {
        let resolved_target = possible_meaning;
        // if let Some(name) = name {
        //     if let Some(value) = self.items[base].scope.get(name) {
        //         resolved_target = *value;
        //     }
        // }
        todo!();
        // let resolved_target = self.substitute(resolved_target, new_subs).unwrap();
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
        value: ConstructId<'x>,
    ) -> Substitutions<'x> {
        for (dep, _) in &*deps {
            let _dep = *dep;
            let subbed_dep = todo!(); //self.substitute(dep.var_item, previous_subs).unwrap();
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
