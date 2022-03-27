use std::collections::HashSet;

use super::{dependencies::DepResStackFrame, discover_equality::Equal, Environment, ItemId};
use crate::{
    constructs::{
        recursion::CRecursion, substitution::Substitutions, Construct, GenInvResult, ItemDefinition,
    },
    scope::{LookupInvariantError, LookupInvariantResult, Scope},
};

impl<'x> Environment<'x> {
    fn arrest_recursion_impl(&mut self, of: ItemId, stack: &mut Vec<ItemId>) {
        stack.push(of);
        match &self.get_item(of).definition {
            &ItemDefinition::Other(other) => {
                if stack.contains(&other) {
                    self.items[of].definition =
                        ItemDefinition::Resolved(Box::new(CRecursion::new(other)));
                } else {
                    self.arrest_recursion_impl(other, stack);
                }
            }
            ItemDefinition::Resolved(con) => {
                for contained in con.contents() {
                    self.arrest_recursion_impl(contained, stack);
                }
            },
            ItemDefinition::Unresolved(..) => (),
        }
        assert_eq!(stack.pop(), Some(of));
    }

    pub fn arrest_recursion(&mut self, of: ItemId) {}
}
