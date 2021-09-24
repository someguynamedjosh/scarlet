use super::ReduceOptions;
use crate::{
    shared::{Item, ItemId},
    stage4::structure::Environment,
};

impl Environment {
    fn reduce_condition(&mut self, opts: ReduceOptions, cond: ItemId) -> Result<bool, ItemId> {
        let rcond = self.reduce(opts.with_item(cond));

        match &self.items[rcond.0].definition {
            Item::PrimitiveValue(val) => Ok(val.expect_bool()),
            _ => Err(rcond),
        }
    }

    pub fn reduce_pick(
        &mut self,
        opts: ReduceOptions,
        initial_clause: (ItemId, ItemId),
        elif_clauses: Vec<(ItemId, ItemId)>,
        else_clause: ItemId,
    ) -> ItemId {
        // Stores clauses where we don't know if they are true or false yet.
        let mut unknown_clauses = Vec::new();

        match self.reduce_condition(opts, initial_clause.0) {
            Ok(true) => {
                debug_assert_eq!(unknown_clauses.len(), 0);
                return self.reduce(opts.with_item(initial_clause.1));
            }
            Ok(false) => (),
            Err(reduced) => {
                unknown_clauses.push((reduced, self.reduce(opts.with_item(initial_clause.1))))
            }
        }

        for (cond, val) in elif_clauses {
            match self.reduce_condition(opts, cond) {
                Ok(true) => {
                    let val = self.reduce(opts.with_item(val));
                    if unknown_clauses.is_empty() {
                        // Only return if we know for sure no previous clauses will be used.
                        return val;
                    }
                }
                Ok(false) => (),
                Err(reduced) => unknown_clauses.push((reduced, self.reduce(opts.with_item(val)))),
            }
        }

        let else_value = self.reduce(opts.with_item(else_clause));
        if unknown_clauses.is_empty() {
            return else_value;
        }

        let item = Item::Pick {
            initial_clause: unknown_clauses[0],
            elif_clauses: unknown_clauses.into_iter().skip(1).collect(),
            else_clause: else_value,
        };
        let typ = self.items[else_value.0].typee.unwrap();
        self.insert_with_type(item, typ)
    }
}
