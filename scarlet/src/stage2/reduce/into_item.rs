use crate::stage2::{
    matchh::MatchResult,
    structure::{
        BuiltinOperation, BuiltinValue, Condition, Definition, Environment, ItemId,
        StructField, Substitutions, VarType,
    },
};

impl<'x> Environment<'x> {
    pub(super) fn reduce_substitution(
        &mut self,
        base: ItemId<'x>,
        subs: Substitutions<'x>,
        item: ItemId<'x>,
    ) -> ItemId<'x> {
        let subbed = self.substitute(base, &subs).unwrap();
        if subbed == item {
            subbed
        } else {
            self.reduce(subbed)
        }
    }

    pub(super) fn reduce_member(
        &mut self,
        base: ItemId<'x>,
        name: String,
        item: ItemId<'x>,
    ) -> ItemId<'x> {
        let base = self.reduce(base);
        if let Definition::Struct(fields) = self.get_definition(base) {
            for (index, field) in fields.iter().enumerate() {
                if field.name == Some(&name) || format!("{}", index) == name {
                    return field.value;
                }
            }
            todo!("Nice error, no field named {}", name)
        } else {
            let def = Definition::Member(base, name);
            self.item_with_new_definition(item, def, false)
        }
    }

    pub(super) fn reduce_match(
        &mut self,
        base: ItemId<'x>,
        else_value: ItemId<'x>,
        conditions: Vec<Condition<'x>>,
        item: ItemId<'x>,
    ) -> ItemId<'x> {
        let base = self.reduce(base);
        let mut new_conditions = Vec::new();
        let mut else_value = else_value;
        for condition in conditions {
            match self.matches(base, condition.pattern) {
                MatchResult::Match(subs) => {
                    else_value = self.substitute(condition.value, &subs).unwrap();
                    break;
                }
                MatchResult::NoMatch => (),
                MatchResult::Unknown => new_conditions.push(condition),
            }
        }
        let conditions = new_conditions;
        if conditions.len() == 0 {
            else_value
        } else {
            let def = Definition::Match {
                base,
                conditions,
                else_value,
            };
            self.item_with_new_definition(item, def, false)
        }
    }
}
