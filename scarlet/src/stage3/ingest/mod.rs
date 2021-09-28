use std::collections::HashMap;

use super::structure::ItemDefinition;
use crate::{
    shared::{Definitions, Item, ItemId, Replacements, VarList},
    stage2::{self, structure::UnresolvedItem},
    stage3::structure::Environment,
};

pub fn ingest(src: stage2::structure::Environment) -> Result<Environment, String> {
    let mut ctx = Context {
        src,
        stage2_to_stage3: HashMap::new(),
        stage3_items: Vec::new(),
        next_stage3_id: ItemId(0),
    };
    let mut id = ItemId(0);
    while id.0 < ctx.src.items.len() {
        ctx.convert_iid(id)?;
        id.0 += 1;
    }

    let mut env = Environment::new();
    ctx.stage3_items.sort_unstable_by_key(|k| k.0);
    let items = ctx.stage3_items;
    let mut next_expected_id = ItemId(0);
    for (id, def) in items {
        assert_eq!(id, next_expected_id);
        env.insert(def);
        next_expected_id.0 += 1;
    }

    Ok(env)
}

struct Context {
    src: stage2::structure::Environment,
    stage2_to_stage3: HashMap<ItemId, ItemId>,
    stage3_items: Vec<(ItemId, ItemDefinition)>,
    next_stage3_id: ItemId,
}

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

    fn get_member(&self, from: ItemId, name: &str) -> Result<ItemId, String> {
        let def = self.src.get(from);
        let item = def.definition.as_ref().expect("ICE: Undefined Item");
        if let UnresolvedItem::Just(Item::Defining { base, definitions }) = item {
            if let Ok(member) = self.get_member(*base, name) {
                return Ok(member);
            } else {
                for def in definitions {
                    if def.0 == name {
                        return Ok(def.1);
                    }
                }
            }
        }
        Err(format!("{:?} has no member named {}", from, name))
    }

    fn reserve_new_item(&mut self, s2_id: ItemId) -> ItemId {
        let s3_id = self.next_stage3_id;
        self.next_stage3_id.0 += 1;
        self.stage2_to_stage3.insert(s2_id, s3_id);
        s3_id
    }

    fn convert_unresolved_item(&mut self, item_id: ItemId) -> Result<ItemId, String> {
        let item_def = self.src.get(item_id).clone();
        match item_def.definition.as_ref().expect("ICE: Undefined item") {
            UnresolvedItem::Item(id) => self.convert_iid(*id),
            UnresolvedItem::Just(item) => {
                let new_id = self.reserve_new_item(item_id);
                let citem = self.convert_item(item)?;
                let def = ItemDefinition::from(&item_def, citem);
                self.stage3_items.push((new_id, def));
                Ok(new_id)
            }
            UnresolvedItem::Member { base, name } => self.get_member(*base, name),
        }
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
            Item::FromType { base, vars } => Item::FromType {
                base: self.convert_iid(*base)?,
                vars: self.convert_var_list(vars)?,
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
