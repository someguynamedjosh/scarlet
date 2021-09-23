use super::structure::Environment;
use crate::shared::{Item, ItemId, PrimitiveValue};

impl Environment {
    pub fn display_infos(&self) {
        for info in &self.infos {
            let item = &self.items[info.0];
            self.display_item(&item.base);
            println!();
        }
    }

    pub fn display_item(&self, item: &Item) {
        match item {
            Item::PrimitiveValue(val) => self.display_primitive_value(*val),
            Item::Variable { selff, .. } => self.display_variable(*selff),
            _ => todo!(),
        }
    }

    fn get_item_name(&self, id: ItemId) -> Option<String> {
        self.get_item_name_impl(id, vec![])
    }

    fn get_item_name_impl(&self, id: ItemId, already_checked: Vec<ItemId>) -> Option<String> {
        let mut choices = Vec::new();
        for (index, potential_parent) in self.items.iter().enumerate() {
            let parent_id = ItemId(index);
            if already_checked.contains(&parent_id) {
                continue;
            }
            if let Item::Defining { definitions, .. } = &potential_parent.base {
                if let Some(def_index) = definitions.iter().position(|def| def.1 == id) {
                    let new_checked = [already_checked.clone(), vec![parent_id]].concat();
                    let parent_name = self.get_item_name_impl(parent_id, new_checked);
                    let this_name = &definitions[def_index].0;
                    if let Some(parent_name) = parent_name {
                        choices.push(format!("{}::{}", parent_name, this_name));
                    } else {
                        choices.push(this_name.clone());
                    }
                }
            }
        }
        choices.into_iter().min_by_key(|e| e.len())
    }

    fn display_item_name(&self, id: ItemId) {
        match self.get_item_name(id) {
            Some(name) => print!("{}", name),
            None => print!("anonymous"),
        }
    }

    fn display_variable(&self, selff: ItemId) {
        self.display_item_name(selff)
    }

    fn display_primitive_value(&self, value: PrimitiveValue) {
        match value {
            PrimitiveValue::Bool(val) => {
                if val {
                    print!("true")
                } else {
                    print!("false")
                }
            }
            PrimitiveValue::I32(val) => print!("i32{{{}}}", val),
        }
    }
}
