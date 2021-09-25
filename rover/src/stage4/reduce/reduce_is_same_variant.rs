use super::ReduceOptions;
use crate::{
    shared::{Item, ItemId, PrimitiveValue},
    stage4::structure::Environment,
};

impl Environment {
    pub fn reduce_is_same_variant(
        &mut self,
        opts: ReduceOptions,
        base: ItemId,
        other: ItemId,
    ) -> ItemId {
        let rbase_id = self.reduce(opts.with_item(base));
        let rother_id = self.reduce(opts.with_item(other));
        let rbase = &self.items[rbase_id.0];
        let rother = &self.items[rother_id.0];
        match (&rbase.definition, &rother.definition) {
            (
                Item::InductiveValue {
                    variant_name: base_variant,
                    ..
                },
                Item::InductiveValue {
                    variant_name: other_variant,
                    ..
                },
            ) => {
                let result = base_variant == other_variant;
                self.insert_with_type(
                    Item::PrimitiveValue(PrimitiveValue::Bool(result)),
                    self.bool_type(),
                    opts.defined_in,
                )
            }
            (Item::PrimitiveValue(base_value), Item::PrimitiveValue(other_value)) => {
                let result = base_value == other_value;
                self.insert_with_type(
                    Item::PrimitiveValue(PrimitiveValue::Bool(result)),
                    self.bool_type(),
                    opts.defined_in,
                )
            }
            _ => {
                if base == rbase_id || other == rother_id {
                    opts.item
                } else {
                    let item = Item::IsSameVariant {
                        base: rbase_id,
                        other: rother_id,
                    };
                    let id = self.insert(item, opts.defined_in);
                    self.compute_type(id, vec![]).unwrap();
                    id
                }
            }
        }
    }
}
