use crate::{
    shared::{Item, ItemId},
    stage4::{ingest::VarList, structure::Environment},
};

impl Environment {
    // Collects all variables specified by From items pointed to by the provided ID.
    pub fn get_from_variables(&mut self, typee: ItemId) -> Result<VarList, String> {
        Ok(match &self.items[typee.0].base {
            Item::Defining { base: id, .. } => {
                let id = *id;
                self.get_from_variables(id)?
            }
            Item::FromType { base, vars } => {
                let base = *base;
                let vars = vars.clone();
                let mut result = self.get_from_variables(base)?;
                result.append(&vars[..]);
                result
            }
            Item::Replacing { .. } => todo!(),
            _ => VarList::new(),
        })
    }

    pub fn with_from_vars(&mut self, base: ItemId, from_vars: VarList) -> ItemId {
        if from_vars.len() > 0 {
            self.insert(Item::FromType {
                base,
                vars: from_vars.into_vec(),
            })
        } else {
            base
        }
    }
}
