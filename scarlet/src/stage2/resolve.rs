use super::structure::Substitutions;
use crate::{
    shared::OrderedSet,
    stage2::{
        matchh::MatchResult,
        structure::{BuiltinValue, Definition, Environment, ItemId, Token, VariableInfo},
        transformers,
    },
};

impl<'x> Environment<'x> {
    pub(super) fn resolve(&mut self, item: ItemId<'x>) -> ItemId<'x> {
        if let Definition::Unresolved(token) = self.get_definition(item) {
            let new_def = match token {
                Token::Stream {
                    label: "substitution",
                    contents,
                } => {
                    let mut contents = contents.clone();
                    // let base = contents.remove(0);
                    // let mut subs = contents;
                    todo!()
                    // let new_subs = self.resolve_targets_in_sub(base, &mut
                    // subs);
                }
                Token::Stream {
                    label: "syntax_root",
                    contents,
                } => {
                    let mut contents = contents.clone();
                    transformers::apply_transformers(self, &mut contents, &Default::default());
                    assert_eq!(
                        contents.len(),
                        1,
                        "Nice error, expected a single expression."
                    );
                    Definition::Unresolved(contents.into_iter().next().unwrap())
                }
                Token::Item(other) => return *other,
                Token::Plain(plain) => {
                    if let Ok(int) = plain.parse::<u32>() {
                        Definition::BuiltinValue(BuiltinValue::_32U(int))
                    } else if *plain == "true" {
                        Definition::BuiltinValue(BuiltinValue::Bool(true))
                    } else if *plain == "false" {
                        Definition::BuiltinValue(BuiltinValue::Bool(false))
                    } else {
                        println!("{:#?}", self);
                        todo!()
                    }
                }
                other => {
                    println!("{:#?}", self);
                    todo!("Nice error, cannot convert {:?} into an item", other)
                }
            };
            self.items[item].definition = Some(new_def);
        }
        item
    }

    // fn resolve_targets_in_sub(
    //     &mut self,
    //     base: ItemId<'x>,
    //     subs: &mut [UnresolvedSubstitution<'x>],
    // ) -> Substitutions<'x> {
    //     let mut new_subs = Substitutions::new();
    //     let mut deps = self.get_deps(base);
    //     for sub in &mut *subs {
    //         if let Some(possible_meaning) = sub.target_meaning {
    //             let additional_subs = self.resolve_named_target(
    //                 possible_meaning,
    //                 sub.target_name,
    //                 base,
    //                 sub.value,
    //                 &mut deps,
    //                 &new_subs,
    //             );
    //             new_subs = new_subs.union(additional_subs);
    //         }
    //     }
    //     for sub in &mut *subs {
    //         if sub.target_meaning.is_none() {
    //             let additions = self.resolve_anonymous_target(&mut deps,
    // &new_subs, sub.value);             new_subs = new_subs.union(additions);
    //         }
    //     }
    //     new_subs
    // }

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
