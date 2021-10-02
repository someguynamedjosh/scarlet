use super::context::Context;
use crate::{stage2::structure as s2, stage3::structure as s3};

impl<'a> Context<'a> {
    pub fn ingest_replacements(
        &mut self,
        replacements: s2::ReplacementsId,
    ) -> s3::ReplacementsId {
        let mut result = s3::Replacements::new();
        for (target, value) in self.input[replacements].clone() {
            let target = self.dereference_variable(target);
            let value = self.ingest(value);
            if result.contains_key(&target) {
                todo!("nice error")
            }
            result.insert_no_replace(target, value);
        }
        self.output.replacements.get_or_push(result)
    }

    pub fn ingest_replacements_list(
        &mut self,
        replacements: Vec<s2::ReplacementsId>,
    ) -> Vec<s3::ReplacementsId> {
        replacements
            .into_iter()
            .map(|reps| self.ingest_replacements(reps))
            .collect()
    }

    pub fn ingest_variable(&mut self, variable_id: s2::VariableId) -> s3::VariableId {
        if let Some(result) = self.variable_map.get(&variable_id) {
            return *result;
        }
        let original_type = self.input[variable_id].original_type;
        let ingested_type = self.ingest(original_type);
        // For now, set the definition to the type because we do not yet have an
        // ID for the actual definition.
        let variable = s3::Variable {
            definition: ingested_type,
            original_type: ingested_type,
        };
        let result = self.output.variables.push(variable);
        self.variable_map.insert(variable_id, result);
        result
    }

    pub fn ingest_variant(&mut self, variant_id: s2::VariantId) -> s3::VariantId {
        if let Some(result) = self.variant_map.get(&variant_id) {
            return *result;
        }
        let original_type = self.input[variant_id].original_type;
        let ingested_type = self.ingest(original_type);
        // For now, set the definition to the type because we do not yet have an
        // ID for the actual definition.
        let variant = s3::Variant {
            definition: ingested_type,
            original_type: ingested_type,
        };
        let result = self.output.variants.push(variant);
        self.variant_map.insert(variant_id, result);
        result
    }
}
