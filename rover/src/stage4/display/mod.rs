use super::structure::Environment;
use crate::{shared::ItemId, util::indented};

mod code;
mod name;

impl Environment {
    pub fn display_infos(&self) {
        for (id, item) in self.iter() {
            if let Some(scope) = item.info_requested {
                let repr = self.get_item_code_or_name(id, scope);
                println!("{}", repr);
                if let Some(typee) = item.typee {
                    let type_repr = self.get_item_code_or_name(typee, scope);
                    println!("type_is{{\n    {}\n}}", indented(&type_repr));
                }
            }
        }
    }

    /// Tries to get code. If that fails, gets a name instead.
    pub fn get_item_code_or_name(&self, item_id: ItemId, in_scope: ItemId) -> String {
        if let Some(code) = self.get_item_code(&self.items[item_id.0].definition, in_scope) {
            return code;
        } else if let Some(name) = self.get_item_name(item_id, in_scope) {
            return name;
        } else {
            return format!("anonymous");
        }
    }

    /// Tries to get a name. If that fails, gets code instead.
    pub fn get_item_name_or_code(&self, item_id: ItemId, in_scope: ItemId) -> String {
        if let Some(name) = self.get_item_name(item_id, in_scope) {
            return name;
        } else if let Some(code) = self.get_item_code(&self.items[item_id.0].definition, in_scope) {
            return code;
        } else {
            return format!("anonymous");
        }
    }
}
