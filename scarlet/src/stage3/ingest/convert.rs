use super::helpers::Context;
use crate::{
    shared::{Definitions, Item, ItemId, Replacements, VarList},
    stage2::structure::UnresolvedItem,
    stage3::structure::ItemDefinition,
};

impl Context {
    pub fn convert_iid(&mut self, id: ItemId) -> Result<ItemId, String> {
        if let Some(s3) = self.stage2_to_stage3.get(&id) {
            Ok(*s3)
        } else {
            let new_id = self.convert_unresolved_item(id)?;
            self.stage2_to_stage3.insert(id, new_id);
            Ok(new_id)
        }
    }

    fn convert_unresolved_item(&mut self, item_id: ItemId) -> Result<ItemId, String> {
        let item_def = self.src.get(item_id).clone();
        let id = match item_def.definition.as_ref().expect("ICE: Undefined item") {
            UnresolvedItem::Item(id) => self.convert_iid(*id),
            UnresolvedItem::Just(item) => {
                let new_id = self.reserve_new_item(item_id);
                let citem = self.convert_item(item)?;
                let defined_in = match item_def.defined_in {
                    Some(id) => Some(self.convert_iid(id)?),
                    None => None,
                };
                let def = ItemDefinition::new(item_def.is_scope, citem, defined_in);
                self.stage3_items.push((new_id, def));
                Ok(new_id)
            }
            UnresolvedItem::Member { base, name } => {
                let s2_id = self.get_member(*base, name)?;
                self.convert_iid(s2_id)
            }
        };
        let id = id?;
        if let Some(scope) = item_def.info_requested {
            let cscope = self.convert_iid(scope)?;
            self.info_requests.push((id, cscope));
        }
        Ok(id)
    }

    fn convert_definitions(&mut self, definitions: &Definitions) -> Result<Definitions, String> {
        let mut new_definitions = Definitions::new();
        for (name, val) in definitions {
            new_definitions.insert_no_replace((name.clone(), self.convert_iid(*val)?))
        }
        Ok(new_definitions)
    }

    fn convert_replacements(
        &mut self,
        replacements: &Replacements,
    ) -> Result<Replacements, String> {
        let mut new_replacements = Replacements::new();
        for (target, val) in replacements {
            let target = self.convert_iid(*target)?;
            let val = self.convert_iid(*val)?;
            new_replacements.insert_no_replace((target, val))
        }
        Ok(new_replacements)
    }

    fn convert_var_list(&mut self, var_list: &VarList) -> Result<VarList, String> {
        let mut new_var_list = VarList::new();
        for var in var_list {
            new_var_list.push(self.convert_iid(*var)?)
        }
        Ok(new_var_list)
    }

    fn convert_item(&mut self, item: &Item) -> Result<Item, String> {
        Ok(match item {
            Item::Any { selff, typee } => Item::Any {
                selff: self.convert_iid(*selff)?,
                typee: self.convert_iid(*typee)?,
            },
            Item::BuiltinOperation(..) => todo!(),
            Item::BuiltinValue(val) => Item::BuiltinValue(*val),
            Item::Defining { base, definitions } => Item::Defining {
                base: self.convert_iid(*base)?,
                definitions: self.convert_definitions(definitions)?,
            },
            Item::FromType { base, vals: vars } => Item::FromType {
                base: self.convert_iid(*base)?,
                vals: self.convert_var_list(vars)?,
            },
            Item::Pick { .. } => todo!(),
            Item::Replacing { base, replacements } => Item::Replacing {
                base: self.convert_iid(*base)?,
                replacements: self.convert_replacements(replacements)?,
            },
            Item::TypeIs {
                base_type_only,
                base,
                typee,
            } => Item::TypeIs {
                base_type_only: *base_type_only,
                base: self.convert_iid(*base)?,
                typee: self.convert_iid(*typee)?,
            },
            Item::Variant { selff, typee } => Item::Variant {
                selff: self.convert_iid(*selff)?,
                typee: self.convert_iid(*typee)?,
            },
        })
    }
}
