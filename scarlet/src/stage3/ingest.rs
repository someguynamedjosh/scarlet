mod dereference;

use std::collections::HashMap;

use self::dereference::DereferencedItem;
use crate::{stage2::structure as s2, stage3::structure as s3};

struct Context<'i, 'o> {
    input: &'i s2::Environment,
    output: &'o mut s3::Environment,
    cache: HashMap<s2::ItemId, s3::ValueId>,
    opaques: HashMap<s2::OpaqueId, s3::OpaqueId>,
}

impl<'i, 'o> Context<'i, 'o> {
    fn gpv(&mut self, value: s3::Value) -> s3::ValueId {
        self.output.get_or_push_value(value)
    }

    fn ingest_dereffed(&mut self, dereffed: DereferencedItem) -> s3::ValueId {
        self.ingest_item(dereffed.base)
    }

    fn ingest_opaque(&mut self, id: s2::OpaqueId) -> s3::OpaqueId {
        if let Some(result) = self.opaques.get(&id) {
            *result
        } else {
            let stage2_id = id;
            let result = self
                .output
                .opaque_values
                .push(s3::OpaqueValue { stage2_id });
            self.opaques.insert(id, result);
            result
        }
    }

    fn ingest_item(&mut self, item: s2::ItemId) -> s3::ValueId {
        if let Some(result) = self.cache.get(&item) {
            return *result;
        }
        let result = match &self.input.items[item].item {
            s2::Item::BuiltinOperation(_) => todo!(),
            s2::Item::BuiltinValue(val) => self.gpv(s3::Value::BuiltinValue(*val)),
            s2::Item::Defining { base, definitions } => {
                let base = self.ingest_item(*base);
                self.cache.insert(item, base);
                for (_name, value) in definitions {
                    self.ingest_item(*value);
                }
                base
            }
            s2::Item::From { base, value } => {
                let base = self.ingest_item(*base);
                let value = self.ingest_item(*value);
                self.gpv(s3::Value::From { base, value })
            }
            s2::Item::Identifier(name) => {
                let in_scope = self.input.items[item].parent_scope.unwrap();
                let dereffed = self.dereference_identifier(name, in_scope);
                self.ingest_dereffed(dereffed)
            }
            s2::Item::Match { base, cases } => {
                let base = self.ingest_item(*base);
                let mut new_cases = Vec::new();
                for (target, value) in cases {
                    let target = self.ingest_item(*target);
                    let value = self.ingest_item(*value);
                    new_cases.push((target, value));
                }
                let cases = new_cases;
                self.gpv(s3::Value::Match { base, cases })
            }
            s2::Item::Member { base, name } => {
                let dereffed = self
                    .dereference_member(*base, name)
                    .expect("TODO: Nice error");
                self.ingest_dereffed(dereffed)
            }
            s2::Item::Opaque { class, id, typee } => {
                let class = *class;
                let id = self.ingest_opaque(*id);
                let typee = self.ingest_item(*typee);
                self.gpv(s3::Value::Opaque { class, id, typee })
            }
            s2::Item::Substituting {
                base,
                substitutions,
            } => {
                let base = self.ingest_item(*base);
                let mut new_subs = s3::Substitutions::new();
                for (target, value) in substitutions {
                    new_subs.push((
                        target.map(|x| self.ingest_item(x)),
                        self.ingest_item(*value),
                    ));
                }
                self.gpv(s3::Value::Substituting {
                    base,
                    substitutions: new_subs,
                })
            }
            s2::Item::TypeIs { base, typee } => {
                let base = self.ingest_item(*base);
                let typee = self.ingest_item(*typee);
                self.gpv(s3::Value::TypeIs { base, typee })
            }
        };
        self.cache.insert(item, result);
        result
    }
}

pub fn ingest(input: &s2::Environment, root: s2::ItemId) -> (s3::Environment, s3::ValueId) {
    let mut output = s3::Environment::new();

    let new_root = Context {
        input,
        output: &mut output,
        cache: HashMap::new(),
        opaques: HashMap::new(),
    }
    .ingest_item(root);

    (output, new_root)
}
