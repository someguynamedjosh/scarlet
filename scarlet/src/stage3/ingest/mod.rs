use std::collections::HashMap;

use self::{context::Context, dereference::DereferencedItem};
use crate::{stage2::structure as s2, stage3::structure as s3};

mod context;
mod defining;
mod dereference;
mod opaque;
mod substituting;

pub fn ingest(s2_env: &s2::Environment, input: s2::ItemId) -> (s3::Environment, s3::ValueId) {
    let mut environment = s3::Environment::new();
    // let mut variable_map = HashMap::new();
    // let mut variant_map = HashMap::new();
    let mut ctx = Context {
        environment: &mut environment,
        ingest_map: &mut HashMap::new(),
        opaque_map: &mut HashMap::new(),
        path: Some(s3::Path::new()),
        input: s2_env,
        parent_scopes: Vec::new(),
    };
    let value = ctx.ingest(input);
    (environment, value)
}

impl<'e, 'i> Context<'e, 'i> {
    fn ingest_dereferenced(&mut self, item: DereferencedItem) -> s3::ValueId {
        let base = self.ingest(item.base);
        let mut result = base;
        for (target, value) in item.subs {
            result = self.gpv(s3::Value::Substituting {
                base: result,
                target,
                value,
            });
        }
        result
    }

    pub fn ingest(&mut self, input: s2::ItemId) -> s3::ValueId {
        if let Some(result) = self.ingest_map.get(&input) {
            return *result;
        }
        let result = match &self.input.items[input] {
            s2::Item::BuiltinOperation(op) => {
                let op = op.map(|input| self.child().without_path().ingest(input));
                self.gpv(s3::Value::BuiltinOperation(op))
            }
            s2::Item::BuiltinValue(value) => self.gpv(s3::Value::BuiltinValue(*value)),
            s2::Item::Defining { base, definitions } => {
                self.ingest_defining(definitions, base, input)
            }
            s2::Item::From { base, value } => {
                let (base, value) = (*base, *value);
                let value = self.ingest(value);
                let variables = self.environment.dependencies(value);
                let base = self.ingest(base);
                self.environment.with_from_variables(base, &variables[..])
            }
            s2::Item::Identifier(name) => {
                let dereffed = self.dereference_identifier(name);
                self.ingest_dereferenced(dereffed)
            }
            s2::Item::Match { base, cases: in_cases } => {
                let base = self.ingest(*base);
                let mut cases = Vec::new();
                for (condition, value) in in_cases {
                    let condition = self.ingest(*condition);
                    let value = self.ingest(*value);
                    cases.push((condition, value));
                }
                self.gpv(s3::Value::Match { base, cases })
            }
            s2::Item::Member { base, name } => {
                let dereffed = self.dereference_member(*base, name);
                match dereffed {
                    Some(dereffed) => self.ingest_dereferenced(dereffed),
                    None => todo!("Nice error, no member {} in {:?}", name, base),
                }
            }
            s2::Item::Opaque { class, typee, id } => self.ingest_opaque(class, id, typee),
            s2::Item::Substituting {
                base,
                target,
                value,
            } => self.ingest_substituting(base, target, value),
        };
        self.ingest_map.insert(input, result);
        result
    }
}
