use super::Environment;
use crate::{
    item::{
        definitions::{
            decision::DDecision,
            is_populated_struct::DIsPopulatedStruct,
            structt::{AtomicStructMember, DAtomicStructMember, DPopulatedStruct},
            substitution::DSubstitution,
            variable::DVariable,
        },
        item::ItemPtr,
        resolvable::{from::RFrom, RSubstitution, UnresolvedItemError},
    },
    scope::{SPlain, Scope},
};

impl Environment {
    fn push_and(&mut self, left: ItemPtr, right: ItemPtr, scope: Box<dyn Scope>) -> ItemPtr {
        let and = self.get_language_item("and");
        self.push_unresolved(
            RSubstitution {
                base: and,
                named_subs: vec![].into_iter().collect(),
                anonymous_subs: vec![left, right],
            },
            scope,
        )
    }

    fn define_and(&mut self, original: ItemPtr, left: ItemPtr, right: ItemPtr) {
        let and = self.get_language_item("and");
        self.define_unresolved(
            original,
            RSubstitution {
                base: and,
                named_subs: vec![].into_iter().collect(),
                anonymous_subs: vec![left, right],
            },
        )
    }

    pub fn create_from_dex(&mut self, from: ItemPtr) -> Result<ItemPtr, UnresolvedItemError> {
        let scope = || Box::new(SPlain(from));
        let into = if let Some(from_dex) = self.items[from].from_dex {
            from_dex
        } else {
            self.push_placeholder(scope())
        };
        if self.items[into].definition.is_placeholder() {
            self.items[from].from_dex = Some(into);
            let x = self.get_language_item("x");

            if let Some(structt) =
                self.get_and_downcast_construct_definition::<DPopulatedStruct>(from)?
            {
                let structt = structt.clone();

                let is_populated_struct = self.push_construct(DIsPopulatedStruct::new(x), scope());

                let x_value = DAtomicStructMember(x, AtomicStructMember::Value);
                let x_value = self.push_construct(x_value, scope());
                let value_from_value = RFrom {
                    left: x_value,
                    right: structt.get_value(),
                };
                let value_from_value = self.push_unresolved(value_from_value, scope());

                let x_rest = DAtomicStructMember(x, AtomicStructMember::Rest);
                let x_rest = self.push_construct(x_rest, scope());
                let rest_from_rest = RFrom {
                    left: x_rest,
                    right: structt.get_rest(),
                };
                let rest_from_rest = self.push_unresolved(rest_from_rest, scope());

                let first_two = self.push_and(is_populated_struct, value_from_value, scope());
                self.define_and(into, first_two, rest_from_rest);
            } else if let Some(var) =
                self.get_and_downcast_construct_definition::<DVariable>(from)?
            {
                let id = var.get_id();
                let var = self.get_variable(id);
                let invs = Vec::from(var.get_invariants());
                let deps = Vec::from(var.get_dependencies());
                if deps.len() > 0 {
                    todo!();
                }
                let truee = self.get_language_item("true");

                let statement = if invs.len() == 0 {
                    truee
                } else {
                    let mut statement = invs[0];
                    for &part in &invs[1..] {
                        statement = self.push_and(statement, part, scope());
                    }
                    statement
                };

                let subs = vec![(id, x)].into_iter().collect();
                let con = DSubstitution::new_unchecked(into, statement, subs);
                self.define_item(into, con);
            } else {
                let truee = self.get_language_item("true");
                let falsee = self.get_language_item("false");
                let equal = DDecision::new(x, from, truee, falsee);
                self.define_item(into, equal);
            }
        }
        Ok(self.push_other(into, scope()))
    }
}
