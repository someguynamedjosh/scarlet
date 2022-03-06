use super::{Environment, UnresolvedItemError};
use crate::{
    constructs::{
        decision::CDecision,
        is_populated_struct::CIsPopulatedStruct,
        structt::{AtomicStructMember, CAtomicStructMember, CPopulatedStruct},
        substitution::CSubstitution,
        variable::CVariable,
        with_dependencies::CWithDependencies,
        ItemId,
    },
    resolvable::{from::RFrom, RSubstitution},
    scope::{SPlain, Scope},
};

impl<'x> Environment<'x> {
    fn push_and(
        &mut self,
        left: ItemId,
        right: ItemId,
        scope: Box<dyn Scope>,
    ) -> ItemId {
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

    fn define_and(&mut self, original: ItemId, left: ItemId, right: ItemId) {
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

    pub fn create_from_dex(
        &mut self,
        from: ItemId,
    ) -> Result<ItemId, UnresolvedItemError> {
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
                self.get_and_downcast_construct_definition::<CPopulatedStruct>(from)?
            {
                let structt = structt.clone();

                let is_populated_struct = self.push_construct(CIsPopulatedStruct::new(x), scope());

                let x_value = CAtomicStructMember(x, AtomicStructMember::Value);
                let x_value = self.push_construct(x_value, scope());
                let value_from_value = RFrom {
                    left: x_value,
                    right: structt.get_value(),
                };
                let value_from_value = self.push_unresolved(value_from_value, scope());

                let x_rest = CAtomicStructMember(x, AtomicStructMember::Rest);
                let x_rest = self.push_construct(x_rest, scope());
                let rest_from_rest = RFrom {
                    left: x_rest,
                    right: structt.get_rest(),
                };
                let rest_from_rest = self.push_unresolved(rest_from_rest, scope());

                let first_two = self.push_and(is_populated_struct, value_from_value, scope());
                self.define_and(into, first_two, rest_from_rest);
            } else if let Some(var) =
                self.get_and_downcast_construct_definition::<CVariable>(from)?
            {
                let id = var.get_id();
                let var = self.get_variable(id);
                let invs = Vec::from(var.get_invariants());
                let deps = Vec::from(var.get_dependencies());
                if deps.len() > 0 {
                    todo!();
                }
                let reorder_inv_deps = |inv: ItemId| CWithDependencies::new(inv, vec![from]);
                let truee = self.get_language_item("true");

                let statement = if invs.len() == 0 {
                    truee
                } else {
                    let con = reorder_inv_deps(invs[0]);
                    let mut statement = self.push_construct(con, scope());
                    for &inv in &invs[1..] {
                        let con = reorder_inv_deps(inv);
                        let part = self.push_construct(con, scope());
                        statement = self.push_and(statement, part, scope());
                    }
                    statement
                };

                let subs = vec![(id, x)].into_iter().collect();
                let con = CSubstitution::new_unchecked(statement, subs);
                self.define_item(into, con);
            } else {
                let truee = self.get_language_item("truee");
                let falsee = self.get_language_item("falsee");
                let equal = CDecision::new(x, from, truee, falsee);
                self.define_item(into, equal);
            }
        }
        Ok(into)
    }
}
