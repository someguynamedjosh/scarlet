use std::collections::HashMap;

use self::{context::Context, dereference::DereferencedItem};
use crate::{stage2::structure as s2, stage4::structure as s3};

mod context;
mod defining;
mod dereference;
mod opaque;
mod substituting;

pub fn ingest(s2_env: &s2::Environment, input: s2::ItemId) -> (s3::Environment, s3::ValueId) {
    let mut environment = s3::Environment::new();
    // let mut variable_map = HashMap::new();
    // let mut instance_map = HashMap::new();
    let mut ctx = Context {
        environment: &mut environment,
        ingest_map: HashMap::new(),
        opaque_map: HashMap::new(),
        input: s2_env,
        stack: Vec::new(),
    };
    let value = ctx.ingest(input);
    (environment, value)
}

impl<'e, 'i> Context<'e, 'i> {
    fn ingest_dereferenced(&mut self, item: DereferencedItem) -> s3::ValueId {
        let base = self.ingest(item.base);
        if item.subs.len() == 0 {
            base
        } else {
            self.gpv(s3::Value::Substituting {
                base,
                substitutions: item.subs,
            })
        }
    }

    pub fn ingest(&mut self, input: s2::ItemId) -> s3::ValueId {
        if let Some(result) = self.ingest_map.get(&input) {
            return *result;
        }
        if self.stack.contains(&input) {
            return self.environment.gpv(s3::Value::Placeholder(input));
        }
        self.stack.push(input);
        let mut referenced = false;
        let result = match &self.input.items[input].item {
            s2::Item::BuiltinOperation(op) => {
                let op = op.map(|input| self.ingest(input));
                self.gpv(s3::Value::BuiltinOperation(op))
            }
            s2::Item::BuiltinValue(value) => self.gpv(s3::Value::BuiltinValue(*value)),
            s2::Item::Defining { base, definitions } => {
                referenced = true;
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
                referenced = true;
                let dereffed = self.dereference_identifier(name, input);
                self.ingest_dereferenced(dereffed)
            }
            s2::Item::Match {
                base,
                cases: in_cases,
            } => {
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
                referenced = true;
                let dereffed = self.dereference_member(*base, name);
                match dereffed {
                    Some(dereffed) => self.ingest_dereferenced(dereffed),
                    None => todo!("Nice error, no member {} in {:?}", name, base),
                }
            }
            s2::Item::Opaque { class, typee, id } => self.ingest_opaque(class, id, typee),
            s2::Item::Substituting {
                base,
                substitutions,
            } => self.ingest_substituting(base, substitutions),
            s2::Item::TypeIs { base, typee } => {
                let base = self.ingest(*base);
                let typee = self.ingest(*typee);
                let typee = self.environment.reduce(typee);
                self.environment.values[base].cached_type = Some(typee);
                base
            }
        };
        self.ingest_map.insert(input, result);
        if referenced {
            &mut self.environment.values[result].referenced_at
        } else {
            &mut self.environment.values[result].defined_at
        }
        .insert_or_replace(input, ());
        if self.input.items[input].display_requested {
            self.environment.values[result]
                .display_requested_from
                .insert_or_replace(input, ());
        }
        let popped = self.stack.pop();
        debug_assert_eq!(popped, Some(input));
        result
    }
}
