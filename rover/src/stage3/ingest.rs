use crate::{
    stage2::structure::{self as stage2, Definitions, ItemId, Replacements},
    stage3::structure::{Environment, Item},
};
use std::collections::HashMap;

pub fn ingest(src: &stage2::Environment) -> Result<Environment, String> {
    let mut new_id = ItemId(0);
    let mut id_map = HashMap::new();
    let mut to_convert = Vec::new();
    for (id, item) in src.iter() {
        let item = item.as_ref().unwrap();
        if convertible(item) {
            id_map.insert(id, new_id);
            to_convert.push(id);
            new_id.0 += 1
        }
    }
    let mut env = Environment::new();
    let mut ctx = IngestionContext {
        id_map,
        src,
        env: &mut env,
    };
    for id in to_convert {
        let def = src.definition_of(id).as_ref().unwrap();
        let converted = ctx.convert_item(def)?;
        ctx.env.insert(converted);
    }
    Ok(env)
}

struct IngestionContext<'a> {
    id_map: HashMap<ItemId, ItemId>,
    src: &'a stage2::Environment,
    env: &'a mut Environment,
}

/// Returns true if calling convert_item(item) will not panic.
fn convertible(item: &stage2::Item) -> bool {
    match item {
        stage2::Item::Item(..) | stage2::Item::Member { .. } => false,
        _ => true,
    }
}

impl<'a> IngestionContext<'a> {
    /// Returns a new item with convert_iid applied to all its referenced ids.
    fn convert_item(&mut self, item: &stage2::Item) -> Result<Item, String> {
        Ok(match item {
            stage2::Item::Defining { base, definitions } => Item::Defining {
                base: self.convert_iid(*base)?,
                definitions: self.convert_defs(definitions)?,
            },
            stage2::Item::FromType { base, vars } => Item::FromType {
                base: self.convert_iid(*base)?,
                vars: self.convert_iids(vars)?,
            },
            stage2::Item::GodType => Item::GodType,
            stage2::Item::InductiveType(id) => Item::InductiveType(self.convert_iid(*id)?),
            stage2::Item::InductiveValue {
                records,
                typee,
                variant_name,
            } => Item::InductiveValue {
                records: self.convert_iids(records)?,
                typee: self.convert_iid(*typee)?,
                variant_name: variant_name.clone(),
            },
            stage2::Item::Item(..) | stage2::Item::Member { .. } => panic!("Cannot convert these"),
            stage2::Item::PrimitiveType(pt) => Item::PrimitiveType(*pt),
            stage2::Item::PrimitiveValue(pv) => Item::PrimitiveValue(*pv),
            stage2::Item::Replacing { base, replacements } => Item::Replacing {
                base: self.convert_iid(*base)?,
                replacements: self.convert_reps(replacements)?,
            },
            stage2::Item::Variable { selff, typee } => Item::Variable {
                selff: self.convert_iid(*selff)?,
                typee: self.convert_iid(*typee)?,
            },
        })
    }

    fn convert_defs(&mut self, defs: &[(String, ItemId)]) -> Result<Definitions, String> {
        let mut result = Vec::new();
        for (name, def) in defs {
            result.push((name.clone(), self.convert_iid(*def)?));
        }
        Ok(result)
    }

    fn convert_reps(&mut self, reps: &[(ItemId, ItemId)]) -> Result<Replacements, String> {
        let mut result = Vec::new();
        for (target, val) in reps {
            result.push((self.convert_iid(*target)?, self.convert_iid(*val)?));
        }
        Ok(result)
    }

    fn convert_iids(&mut self, ids: &[ItemId]) -> Result<Vec<ItemId>, String> {
        let mut result = Vec::new();
        for id in ids {
            result.push(self.convert_iid(*id)?);
        }
        Ok(result)
    }

    /// Applies dereferencing and id_map to the provided item id.
    fn convert_iid(&mut self, id: ItemId) -> Result<ItemId, String> {
        let deref = self.dereference_iid(id)?;
        Ok(*self.id_map.get(&deref).unwrap())
    }

    fn get_member(&mut self, base: ItemId, name: &String) -> Result<ItemId, String> {
        let base = self.dereference_iid(base)?;
        let og_base = base;
        match self.src.definition_of(base).as_ref().unwrap() {
            stage2::Item::Defining { base, definitions } => {
                if let Ok(member) = self.get_member(*base, name) {
                    return Ok(member);
                }
                for (cname, cdef) in definitions {
                    if cname == name {
                        return Ok(*cdef);
                    }
                }
                Err(format!("{:?} has no member named {}", og_base, name))
            }
            stage2::Item::Item(..)
            | stage2::Item::Member { .. }
            | stage2::Item::Replacing { .. } => unreachable!(),
            _ => Err(format!("{:?} has no members", name)),
        }
    }

    /// Returns the target of the item if it is a reference to another item or member.
    fn dereference_iid(&mut self, id: ItemId) -> Result<ItemId, String> {
        match self.src.definition_of(id).as_ref().unwrap() {
            stage2::Item::Item(id) => self.dereference_iid(*id),
            stage2::Item::Member { base, name } => self.get_member(*base, name),
            stage2::Item::Replacing { .. } => todo!(),
            _ => Ok(id),
        }
    }
}
