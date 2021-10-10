use std::collections::HashMap;

use crate::{stage2::structure as s2, stage3::structure as s3};

struct Context<'i> {
    input: &'i s3::Environment,
    output: s3::Environment,
    map: HashMap<s3::ValueId, s3::ValueId>,
}

impl<'i> Context<'i> {
    fn ingest(&mut self, in_id: s3::ValueId) {
        let value = self.input.values[in_id].value.clone().unwrap();
        let pushed = self.output.get_or_push_value(value);
        self.map.insert(in_id, pushed);
    }

    fn dedup_id(map: &HashMap<s3::ValueId, s3::ValueId>, id: &mut s3::ValueId) {
        *id = *map.get(&*id).unwrap()
    }

    fn dedup_value(map: &HashMap<s3::ValueId, s3::ValueId>, value: &mut s3::Value) {
        let dedup = |id| Self::dedup_id(map, id);
        match value {
            s3::Value::BuiltinOperation(_) => todo!(),
            s3::Value::BuiltinValue(..) => (),
            s3::Value::From { base, value } => {
                dedup(base);
                dedup(value);
            }
            s3::Value::Match { base, cases } => {
                dedup(base);
                for (condition, value) in cases {
                    dedup(condition);
                    dedup(value);
                }
            }
            s3::Value::Opaque { typee, .. } => {
                dedup(typee);
            }
            s3::Value::Substituting {
                base,
                substitutions,
            } => {
                dedup(base);
                for (target, value) in substitutions {
                    if let Some(target) = target.as_mut() {
                        dedup(target)
                    }
                    dedup(value);
                }
            }
            s3::Value::TypeIs { base, typee } => {
                dedup(base);
                dedup(typee);
            }
        }
    }
}

impl s3::Environment {
    pub fn dedup(&mut self, root: s3::ValueId) -> s3::ValueId {
        let mut context = Context {
            input: &*self,
            output: s3::Environment::new(),
            map: HashMap::new(),
        };
        for (id, _) in self.values.iter() {
            context.ingest(id);
        }
        let mut output = context.output;
        let map = context.map;
        for (_, value) in &mut output.values {
            Context::dedup_value(&map, value.value.as_mut().unwrap())
        }
        let mut deduped_root = root;
        Context::dedup_id(&map, &mut deduped_root);

        let quantity_reduced = output.values.len() < self.values.len();
        self.values = output.values;
        if quantity_reduced {
            self.dedup(deduped_root)
        } else {
            deduped_root
        }
    }
}
