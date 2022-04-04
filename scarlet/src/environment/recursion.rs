use std::collections::HashSet;

use super::{
    dependencies::DepResStackFrame, discover_equality::Equal, Environment, ItemId,
    UnresolvedItemError,
};
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
            }
            ItemDefinition::Unresolved(..) => (),
        }
        assert_eq!(stack.pop(), Some(of));
    }

    pub(crate) fn arrest_recursion(&mut self, of: ItemId) {
        self.arrest_recursion_impl(of, &mut Vec::new())
    }

    pub(crate) fn evaluation_of_item_recurses_over(
        &mut self,
        of: ItemId,
    ) -> Result<Vec<ItemId>, UnresolvedItemError> {
        if let Some(rec) = self.get_and_downcast_construct_definition::<CRecursion>(of)? {
            Ok(vec![rec.get_base()])
        } else {
            let mut result = Vec::new();
            for content in self.get_item_as_construct(of)?.contents() {
                result.append(&mut self.evaluation_of_item_recurses_over(content)?);
            }
            Ok(result)
        }
    }
}
