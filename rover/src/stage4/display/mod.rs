use super::structure::Environment;
use crate::{shared::ItemId, util::indented};

mod code;
mod name;

#[derive(Clone, Copy)]
struct Context {
    in_scope: ItemId,
    in_type: Option<ItemId>,
}

impl Context {
    fn new(in_scope: ItemId) -> Self {
        Self {
            in_scope,
            in_type: None,
        }
    }

    fn with_in_scope(mut self, in_scope: ItemId) -> Self {
        self.in_scope = in_scope;
        self
    }

    fn with_in_type(mut self, in_type: ItemId) -> Self {
        self.in_type = Some(in_type);
        self
    }
}

impl Environment {
    pub fn display_infos(&self) {
        for (id, item) in self.iter() {
            if let Some(scope) = item.info_requested {
                let name = self
                    .get_item_name(id, scope)
                    .unwrap_or(format!("anonymous"));
                println!("\n{} is", name);
                let repr = self.get_item_code_or_name(id, Context::new(scope));
                println!("{}", repr);
                if let Some(typee) = item.typee {
                    let type_repr = self.get_item_name_or_code(typee, Context::new(scope));
                    println!("type_is{{\n    {}\n}}", indented(&type_repr));
                }
            }
        }
    }

    /// Tries to get code. If that fails, gets a name instead.
    fn get_item_code_or_name(&self, item_id: ItemId, ctx: Context) -> String {
        if let Some(code) = self.get_item_code(&item_id, ctx) {
            return code;
        } else if let Some(name) = self.get_item_name(item_id, ctx.in_scope) {
            return name;
        } else {
            return format!("anonymous");
        }
    }

    /// Tries to get a name. If that fails, gets code instead.
    fn get_item_name_or_code(&self, item_id: ItemId, ctx: Context) -> String {
        if let Some(name) = self.get_item_name(item_id, ctx.in_scope) {
            return name;
        } else if let Some(code) = self.get_item_code(&item_id, ctx) {
            return code;
        } else {
            return format!("anonymous");
        }
    }
}
