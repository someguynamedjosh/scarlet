use itertools::Itertools;

use super::structure::Substitutions;
use crate::{
    shared::OrderedSet,
    stage2::{
        matchh::MatchResult,
        structure::{BuiltinValue, Definition, Environment, ConstructId, Token, VariableInfo},
        transform::{self, ApplyContext},
    },
};

impl<'x> Environment<'x> {
    pub(super) fn resolve(&mut self, item: ConstructId<'x>) -> ConstructId<'x> {
        let parent_scope = self.items[item].parent_scope;
        if let Definition::Unresolved(token) = self.get_definition(item) {
            let new_def = match token {
                Token::Stream {
                    label: "substitute",
                    contents,
                } => {
                    let mut contents = contents.clone();
                    let base = contents.remove(0);
                    let base = self.push_token(base);
                    self.items[base].parent_scope = parent_scope;
                    let mut subs = contents;
                    let new_subs = self.resolve_targets_in_sub(base, &mut subs, parent_scope);
                    Definition::Substitute(base, new_subs)
                }
                Token::Stream {
                    label: "syntax_root",
                    contents,
                } => {
                    let mut contents = contents.clone();
                    let mut context = ApplyContext {
                        env: self,
                        parent_scope: None,
                    };
                    transform::apply_transformers(&mut context, &mut contents, &Default::default());
                    assert_eq!(
                        contents.len(),
                        1,
                        "Nice error, expected a single expression."
                    );
                    Definition::Unresolved(contents.into_iter().next().unwrap())
                }
                Token::Item(other) => return *other,
                Token::Plain(plain) => {
                    let plain = *plain;
                    self.resolve_plain_token(item, plain)
                }
                other => {
                    println!("{:#?}", self);
                    todo!("Nice error, cannot convert {:?} into an item", other)
                }
            };
            self.items[item].base = Some(new_def);
            self.check(item);
        }
        item
    }

    fn resolve_plain_token(&mut self, item: ConstructId<'x>, plain: &str) -> Definition<'x> {
        if let Ok(int) = plain.parse::<u32>() {
            Definition::BuiltinValue(BuiltinValue::_32U(int))
        } else if plain == "true" {
            Definition::BuiltinValue(BuiltinValue::Bool(true))
        } else if plain == "false" {
            Definition::BuiltinValue(BuiltinValue::Bool(false))
        } else {
            let mut maybe_scope = self.items[item].parent_scope;
            while let Some(scope) = maybe_scope {
                if let Some(result) = self.get_member(scope, plain) {
                    return Definition::Unresolved(Token::Item(result));
                }
                maybe_scope = self.items[scope].parent_scope;
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
        let mut new_subs = Substitutions::new();
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
        value: ConstructId<'x>,
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
