use super::structure::{Environment, ItemId, ScopeId};
use crate::util::indented;

pub mod code;
pub mod name;

impl Environment {
    pub fn display_infos(&self) {
        for (id, scope) in &self.info_requests {
            let item = &self[*id];
            let (item, id) = if let Some(id) = item.cached_reduction {
                (&self[id], id)
            } else {
                (item, *id)
            };
            let name = self
                .item_name(id, *scope)
                .unwrap_or(format!("anonymous {:?}", id));
            println!("\n{} is", name);
            let repr = self.item_code_or_name(id, *scope);
            println!("{}", repr);
            if let Some(typee) = item.typee {
                let type_repr = self.item_name_or_code(typee, *scope);
                println!("type_is{{\n    {}\n}}", indented(&type_repr));
            }
        }
    }

    pub fn item_name_or_code(&self, item: ItemId, context: ScopeId) -> String {
        let reduced = self[item].cached_reduction.unwrap_or(item);
        if let Some(name) = self.item_name(reduced, context) {
            name
        } else if let Some(code) = self.item_code(reduced, context) {
            code
        } else {
            format!("anonymous at {:?}", item)
        }
    }

    pub fn item_code_or_name(&self, item: ItemId, context: ScopeId) -> String {
        let reduced = self[item].cached_reduction.unwrap_or(item);
        if let Some(code) = self.item_code(reduced, context) {
            code
        } else if let Some(name) = self.item_name(reduced, context) {
            name
        } else {
            format!("anonymous at {:?}", item)
        }
    }
}
