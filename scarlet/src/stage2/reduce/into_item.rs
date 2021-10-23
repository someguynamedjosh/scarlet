use crate::{
    shared::OrderedMap,
    stage2::{
        reduce::matchh::MatchResult,
        structure::{After, Condition, Definition, Environment, Item, ItemId, Substitution},
    },
};

impl<'x> Environment<'x> {
    pub(super) fn reduce_match(
        &mut self,
        base: ItemId<'x>,
        else_value: ItemId<'x>,
        conditions: Vec<Condition<'x>>,
        original: ItemId<'x>,
    ) -> ItemId<'x> {
        let base = self.reduce(base);
        let mut new_conditions = Vec::new();
        let mut else_value = else_value;
        for condition in conditions.clone() {
            let pattern = self.reduce(condition.pattern);
            // Don't reduce yet as that might lead to needless infinite recursion.
            let value = condition.value;
            match self.matches(base, pattern) {
                MatchResult::Match(subs) => {
                    if subs.len() > 0 {
                        todo!()
                    }
                    // If the match is always true, no need to evaluate further conditions.
                    // This should always be used when the previous conditions fail.
                    else_value = condition.value;
                    break;
                }
                // If the match will never occur, skip it.
                MatchResult::NoMatch => (),
                // If the match might occur, save it for later.
                MatchResult::Unknown => new_conditions.push(Condition { pattern, value }),
            }
        }
        let is_fundamentally_different = conditions != new_conditions;
        if new_conditions.len() == 0 {
            self.reduce(else_value)
        } else {
            let def = Definition::Match {
                base,
                conditions: new_conditions,
                else_value,
            };
            self.item_with_new_definition(original, def, is_fundamentally_different)
        }
    }

    pub(super) fn reduce_member(&mut self, base: ItemId<'x>, member: String) -> ItemId<'x> {
        let rbase = self.reduce(base);
        if let Definition::Struct(fields) = self.definition_of(rbase) {
            for field in fields {
                if let Some(name) = &field.name {
                    if name == &member {
                        return field.value;
                    }
                }
            }
            todo!("Nice error, no member named {:?}", member)
        } else {
            todo!()
        }
    }

    pub(super) fn reduce_other(&mut self, original: ItemId<'x>, base: ItemId<'x>) -> ItemId<'x> {
        if self.get_afters(original) == self.get_afters(base) {
            self.reduce(base)
        } else {
            let base = self.reduce(base);
            let base = self.items[base].clone();
            let mut result = base;
            result.cached_reduction = None;
            result.dependencies = None;
            result.after = After::AllVars(self.get_afters(original));
            self.items.get_or_push(result)
        }
    }

    pub(super) fn reduce_substitution(
        &mut self,
        subs: Vec<Substitution<'x>>,
        base: ItemId<'x>,
        original: ItemId<'x>,
    ) -> ItemId<'x> {
        let mut final_subs = OrderedMap::new();
        for sub in subs {
            match self.matches(sub.value, sub.target.unwrap()) {
                MatchResult::Match(subs) => final_subs = final_subs.union(subs),
                MatchResult::NoMatch => {
                    todo!("Nice error, argument will definitely not match what it is assigned to.")
                }
                MatchResult::Unknown => {
                    todo!("Nice error, argument might not match what it is assigned to.")
                }
            }
        }
        let base = self.reduce(base);
        let subbed = self.with_fresh_query_stack(|this| this.substitute(base, &final_subs));
        if let Some(subbed) = subbed {
            let shown_from = self.items[original].shown_from.clone();
            self.items[subbed].shown_from = shown_from;
            self.reduce(subbed)
        } else {
            original
        }
    }
}
