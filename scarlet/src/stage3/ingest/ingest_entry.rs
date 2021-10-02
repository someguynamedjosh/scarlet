use super::{context::Context, dereference::DereferencedItem};
use crate::{stage2::structure as s2, stage3::structure as s3};

impl<'a> Context<'a> {
    pub fn ingest(&mut self, value_id: s2::ValueId) -> s3::ValueId {
        if let Some(result) = self.value_map.get(&value_id) {
            return *result;
        }
        let (base, reps) = self.dereference_value(value_id);
        let base = self.ingest_dereferenced_base(base);
        let result = self.add_replacements(base, reps);
        self.value_map.insert(value_id, result);
        result
    }

    pub fn add_replacements(
        &mut self,
        base: s3::ValueId,
        replacements: Vec<s2::ReplacementsId>,
    ) -> s3::ValueId {
        if replacements.is_empty() {
            base
        } else {
            let replacements = self.ingest_replacements_list(replacements);
            let value = s3::Value::Replacing { base, replacements };
            self.output.values.get_or_push(value)
        }
    }

    pub fn ingest_dereferenced_base(&mut self, base: s2::ValueId) -> s3::ValueId {
        let value = self.input[base].as_ref().expect("ICE: Undefined item");
        match value {
            s2::Value::Any { variable } => {
                let variable = *variable;
                self.do_variable(variable)
            }
            s2::Value::BuiltinOperation(op) => {
                let op = op.clone();
                self.do_op(op)
            }
            s2::Value::BuiltinValue(value) => {
                let value = s3::Value::BuiltinValue(*value);
                self.output.values.get_or_push(value)
            }
            s2::Value::From { base, values } => {
                let (base, values) = (*base, values.clone());
                self.do_from(base, values)
            }
            s2::Value::Identifier { .. } => unreachable!(),
            s2::Value::Member { .. } => unreachable!(),
            s2::Value::Replacing { .. } => unreachable!(),
            s2::Value::Variant { variant } => {
                let variant = *variant;
                self.do_variant(variant)
            }
        }
    }

    fn do_from(&mut self, base: s2::ValueId, values: Vec<s2::ValueId>) -> s3::ValueId {
        let base = self.ingest(base);
        let total_deps = self.get_from_deps(values);
        if total_deps.len() == 0 {
            return base;
        }
        let value = s3::Value::From {
            base,
            variables: total_deps,
        };
        self.output.values.get_or_push(value)
    }

    fn get_from_deps(&mut self, values: Vec<s2::ValueId>) -> s3::Variables {
        let mut total_deps = s3::Variables::new();
        for value in values {
            let value = self.ingest(value);
            let value_deps = self.get_dependencies(value);
            total_deps = total_deps.union(value_deps);
        }
        total_deps
    }

    fn get_dependencies(&self, of: s3::ValueId) -> s3::Variables {
        match &self.output[of] {
            s3::Value::Any { variable } => vec![(*variable, ())].into_iter().collect(),
            s3::Value::BuiltinOperation(op) => op
                .inputs()
                .into_iter()
                .flat_map(|input| self.get_dependencies(input).into_iter())
                .collect(),
            s3::Value::BuiltinValue(..) => s3::Variables::new(),
            s3::Value::From { base, variables } => {
                self.get_dependencies(*base).difference(variables)
            }
            s3::Value::Replacing { base, replacements } => {
                let mut keys_to_remove = s3::Variables::new();
                for rep in replacements {
                    for (target, _) in &self.output[*rep] {
                        keys_to_remove.insert_or_replace(*target, ());
                    }
                }
                self.get_dependencies(*base).difference(&keys_to_remove)
            }
            s3::Value::Variant { .. } => s3::Variables::new(),
        }
    }

    fn do_op(&mut self, op: s2::BuiltinOperation<s2::ValueId>) -> s3::ValueId {
        let op = op.map(|arg| self.ingest(arg));
        let value = s3::Value::BuiltinOperation(op);
        self.output.values.get_or_push(value)
    }

    fn do_variable(&mut self, variable: s2::VariableId) -> s3::ValueId {
        let variable = self.ingest_variable(variable);
        let value = s3::Value::Any { variable };
        let result = self.output.values.get_or_push(value);
        self.output.variables[variable].definition = result;
        result
    }

    fn do_variant(&mut self, variant: s2::VariantId) -> s3::ValueId {
        let variant = self.ingest_variant(variant);
        let value = s3::Value::Variant { variant };
        let result = self.output.values.get_or_push(value);
        self.output.variants[variant].definition = result;
        result
    }
}
