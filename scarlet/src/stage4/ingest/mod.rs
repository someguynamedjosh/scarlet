use std::collections::HashMap;

use self::{context::Context, dereference::DereferencedItem};
use crate::{stage2::structure as s3, stage4::structure as s4};

mod context;
mod defining;
mod dereference;
mod opaque;
mod substituting;

pub fn ingest(s2_env: &s3::Environment, input: s3::ItemId) -> (s4::Environment, s4::ValueId) {
    let mut environment = s4::Environment::new();
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
    fn ingest_dereferenced(&mut self, item: DereferencedItem) -> s4::ValueId {
        let base = self.ingest(item.base);
        if item.subs.len() == 0 {
            base
        } else {
            self.gpv(s4::Value::Substituting {
                base,
                substitutions: item.subs,
            })
        }
    }

    pub fn ingest(&mut self, input: s3::ItemId) -> s4::ValueId {
        if let Some(result) = self.ingest_map.get(&input) {
            return *result;
        }
        if self.stack.contains(&input) {
            return self.environment.gpv(s4::Value::Placeholder(input));
        }
        self.stack.push(input);
        let mut referenced = false;
        let result = match &self.input.items[input].item {
            s3::Item::BuiltinOperation(op) => {
                let op = op.map(|input| self.ingest(input));
                self.gpv(s4::Value::BuiltinOperation(op))
            }
            s3::Item::BuiltinValue(value) => self.gpv(s4::Value::BuiltinValue(*value)),
            s3::Item::Defining { base, definitions } => {
                referenced = true;
                self.ingest_defining(definitions, base, input)
            }
            s3::Item::From { base, value } => {
                let (base, value) = (*base, *value);
                let value = self.ingest(value);
                let variables = self.environment.dependencies(value);
                let base = self.ingest(base);
                self.environment.with_from_variables(base, &variables[..])
            }
            s3::Item::Identifier(name) => {
                referenced = true;
                let dereffed = self.dereference_identifier(name, input);
                self.ingest_dereferenced(dereffed)
            }
            s3::Item::Match {
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
                self.gpv(s4::Value::Match { base, cases })
            }
            s3::Item::Member { base, name } => {
                referenced = true;
                let dereffed = self.dereference_member(*base, name);
                match dereffed {
                    Some(dereffed) => self.ingest_dereferenced(dereffed),
                    None => todo!("Nice error, no member {} in {:?}", name, base),
                }
            }
            s3::Item::Opaque { class, typee, id } => self.ingest_opaque(class, id, typee),
            s3::Item::Substituting {
                base,
                substitutions,
            } => self.ingest_substituting(base, substitutions),
            s3::Item::TypeIs { base, typee } => {
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
