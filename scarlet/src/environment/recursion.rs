use std::collections::HashSet;

use super::Environment;
use crate::{
    item::{resolvable::UnresolvedItemError, item::ItemPtr, definition::ItemDefinition, definitions::recursion::DRecursion},
    scope::{LookupInvariantError, LookupInvariantResult, Scope},
};

impl Environment {
    fn arrest_recursion_impl(&mut self, of: ItemPtr, stack: &mut Vec<ItemPtr>) {
        stack.push(of);
        match &self.get_item(of).definition {
            &ItemDefinition::Other(other) => {
                if stack.contains(&other) {
                    self.items[of].definition =
                        ItemDefinition::Resolved(Box::new(DRecursion::new(other)));
                } else {
                    self.arrest_recursion_impl(other, stack);
                }
            }
            ItemDefinition::Resolved(con) => {
                for contained in con.contents() {
                    self.arrest_recursion_impl(contained, stack);
                }
            }
            ItemDefinition::Placeholder => (),
        }
        assert_eq!(stack.pop(), Some(of));
    }

    pub(crate) fn arrest_recursion(&mut self, of: ItemPtr) {
        self.arrest_recursion_impl(of, &mut Vec::new())
    }

    pub(crate) fn evaluation_of_item_recurses_over(
        &mut self,
        of: ItemPtr,
    ) -> Result<Vec<ItemPtr>, UnresolvedItemError> {
        if let Some(rec) = self.get_and_downcast_construct_definition::<DRecursion>(of)? {
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
