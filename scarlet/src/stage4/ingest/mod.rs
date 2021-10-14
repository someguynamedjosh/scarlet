use std::collections::HashMap;

use self::context::Context;
use crate::{stage3::structure as s3, stage4::structure as s4};

mod context;
mod opaque;
mod substituting;

pub fn ingest(s3_env: &s3::Environment, root: s3::ValueId) -> (s4::Environment, s4::ValueId) {
    let mut environment = s4::Environment::new();
    // let mut variable_map = HashMap::new();
    // let mut instance_map = HashMap::new();
    let mut id = s3_env.values.first().unwrap();
    let mut ctx = Context {
        environment: &mut environment,
        ingest_map: HashMap::new(),
        opaque_map: HashMap::new(),
        input: s3_env,
        stack: Vec::new(),
    };
    loop {
        ctx.ingest(id);
        match ctx.input.values.next(id) {
            Some(new) => id = new,
            None => break,
        }
    }
    for (_, value) in &mut ctx.environment.values {
        if let s4::Value::SelfReference {
            original_id,
            self_id,
        } = &mut value.value
        {
            *self_id = Some(*ctx.ingest_map.get(&*original_id).unwrap());
        }
    }
    let new_root = ctx.ingest(root);
    (environment, new_root)
}

impl<'e, 'i> Context<'e, 'i> {
    pub fn ingest(&mut self, input: s3::ValueId) -> s4::ValueId {
        if let Some(result) = self.ingest_map.get(&input) {
            return *result;
        }
        if self.stack.contains(&input) {
            return self.gpv(s4::Value::SelfReference {
                original_id: input,
                self_id: None,
            });
        }
        self.stack.push(input);
        let result = match self.input.values[input].value.clone().unwrap() {
            s3::Value::BuiltinOperation(op) => {
                let op = op.map(|input| self.ingest(input));
                self.gpv(s4::Value::BuiltinOperation(op))
            }
            s3::Value::BuiltinValue(value) => self.gpv(s4::Value::BuiltinValue(value)),
            s3::Value::From { base, value } => {
                let base = self.ingest(base);
                let value = self.ingest(value);
                self.gpv(s4::Value::From { base, value })
            }
            s3::Value::Match {
                base,
                cases: in_cases,
            } => {
                let base = self.ingest(base);
                let mut cases = Vec::new();
                for (condition, value) in in_cases {
                    let condition = self.ingest(condition);
                    let value = self.ingest(value);
                    cases.push((condition, value));
                }
                self.gpv(s4::Value::Match { base, cases })
            }
            s3::Value::Opaque { class, typee, id } => self.ingest_opaque(class, id, typee),
            s3::Value::Substituting {
                base,
                substitutions,
            } => self.ingest_substituting(base, substitutions),
            s3::Value::TypeIs { base, typee } => {
                let base = self.ingest(base);
                let typee = self.ingest(typee);
                if self.environment.values[base].cached_type.is_some() {
                    todo!()
                }
                self.environment.values[base].cached_type = Some(typee);
                base
            }
        };
        self.ingest_map.insert(input, result);
        // self.environment.values[result]
        //     .defined_at
        //     .insert_or_replace(input, ());
        // if self.input.values[input].display_requested {
        //     self.environment.values[result]
        //         .display_requested_from
        //         .insert_or_replace(input, ());
        // }
        let popped = self.stack.pop();
        debug_assert_eq!(popped, Some(input));
        result
    }
}
