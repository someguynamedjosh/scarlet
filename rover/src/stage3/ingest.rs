use crate::{
    stage2::structure::{
        self as stage2, Definitions, IntegerMathOperation, ItemId, PrimitiveOperation, Replacements,
    },
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
        next_unused_id: new_id,
        extra_items: vec![],
    };
    for id in to_convert {
        let def = src.definition_of(id).as_ref().unwrap();
        let converted = ctx.convert_item(def)?;
        ctx.env.insert(converted);
    }
    for item in ctx.extra_items {
        env.insert(item);
    }
    Ok(env)
}

struct IngestionContext<'a> {
    id_map: HashMap<ItemId, ItemId>,
    src: &'a stage2::Environment,
    env: &'a mut Environment,
    next_unused_id: ItemId,
    extra_items: Vec<Item>,
}

/// Returns true if calling convert_item(item) will not panic.
fn convertible(item: &stage2::Item) -> bool {
    match item {
        stage2::Item::Item(..) | stage2::Item::Member { .. } => false,
        _ => true,
    }
}

impl<'a> IngestionContext<'a> {
    fn insert_extra_item(&mut self, item: Item) -> ItemId {
        let id = self.next_unused_id;
        self.extra_items.push(item);
        self.next_unused_id.0 += 1;
        id
    }

    fn convert_integer_op(
        &mut self,
        op: IntegerMathOperation,
    ) -> Result<IntegerMathOperation, String> {
        use IntegerMathOperation as Imo;
        Ok(match op {
            Imo::Add(l, r) => Imo::Add(self.convert_iid(l, true)?, self.convert_iid(r, true)?),
            Imo::Subtract(l, r) => {
                Imo::Subtract(self.convert_iid(l, true)?, self.convert_iid(r, true)?)
            }
        })
    }

    /// Returns a new item with full_convert_iid applied to all its referenced ids.
    fn convert_item(&mut self, item: &stage2::Item) -> Result<Item, String> {
        Ok(match item {
            stage2::Item::Defining { base, definitions } => Item::Defining {
                base: self.full_convert_iid(*base)?,
                definitions: self.convert_defs(definitions)?,
            },
            stage2::Item::FromType { base, vars } => Item::FromType {
                base: self.full_convert_iid(*base)?,
                vars: self.convert_iids(vars)?,
            },
            stage2::Item::GodType => Item::GodType,
            // Don't deref defines on this one so that the type remembers the
            // constructors it can be made with.
            stage2::Item::InductiveType(id) => Item::InductiveType(self.convert_iid(*id, false)?),
            stage2::Item::InductiveValue {
                records,
                typee,
                variant_name,
            } => Item::InductiveValue {
                records: self.convert_iids(records)?,
                typee: self.full_convert_iid(*typee)?,
                variant_name: variant_name.clone(),
            },
            stage2::Item::Item(..) | stage2::Item::Member { .. } => panic!("Cannot convert these"),
            stage2::Item::PrimitiveOperation(op) => match op {
                PrimitiveOperation::I32Math(op) => Item::PrimitiveOperation(
                    PrimitiveOperation::I32Math(self.convert_integer_op(op.clone())?),
                ),
            },
            stage2::Item::PrimitiveType(pt) => Item::PrimitiveType(*pt),
            stage2::Item::PrimitiveValue(pv) => Item::PrimitiveValue(*pv),
            stage2::Item::Replacing { base, replacements } => Item::Replacing {
                base: self.full_convert_iid(*base)?,
                replacements: self.convert_reps(replacements)?,
            },
            stage2::Item::Variable { selff, typee } => Item::Variable {
                selff: self.full_convert_iid(*selff)?,
                typee: self.full_convert_iid(*typee)?,
            },
        })
    }

    fn convert_defs(&mut self, defs: &[(String, ItemId)]) -> Result<Definitions, String> {
        let mut result = Vec::new();
        for (name, def) in defs {
            // Don't dereference defines so we can preserve module structure for
            // when we go backwards from IDs to names.
            result.push((name.clone(), self.convert_iid(*def, false)?));
        }
        Ok(result)
    }

    fn convert_reps(&mut self, reps: &[(ItemId, ItemId)]) -> Result<Replacements, String> {
        let mut result = Vec::new();
        for (target, val) in reps {
            result.push((
                self.full_convert_iid(*target)?,
                self.full_convert_iid(*val)?,
            ));
        }
        Ok(result)
    }

    fn convert_iids(&mut self, ids: &[ItemId]) -> Result<Vec<ItemId>, String> {
        let mut result = Vec::new();
        for id in ids {
            result.push(self.full_convert_iid(*id)?);
        }
        Ok(result)
    }

    fn full_convert_iid(&mut self, id: ItemId) -> Result<ItemId, String> {
        self.convert_iid(id, true)
    }

    /// Applies dereferencing and id_map to the provided item id.
    fn convert_iid(&mut self, id: ItemId, deref_define: bool) -> Result<ItemId, String> {
        let (id, replacements) = self.dereference_iid(id, deref_define)?;
        let id = *self.id_map.get(&id).unwrap();
        if replacements.len() == 0 {
            Ok(id)
        } else {
            let base = id;
            let item = Item::Replacing { base, replacements };
            Ok(self.insert_extra_item(item))
        }
    }

    fn get_member(
        &mut self,
        base: ItemId,
        name: &String,
    ) -> Result<(ItemId, Replacements), String> {
        let (base_id, base_reps) = self.dereference_iid(base, false)?;
        let og_base = base_id;
        match self.src.definition_of(base_id).as_ref().unwrap() {
            stage2::Item::Defining { base, definitions } => {
                if let Ok(member) = self.get_member(*base, name) {
                    return Ok(member);
                }
                for (cname, cdef) in definitions {
                    if cname == name {
                        return Ok((*cdef, base_reps));
                    }
                }
                Err(format!("{:?} has no member named {}", og_base, name))
            }
            stage2::Item::Item(..) | stage2::Item::Member { .. } => unreachable!(),
            stage2::Item::Replacing { .. } => unreachable!("{:?} {:?}", base, base_id),
            _ => Err(format!("{:?} has no members", base)),
        }
    }

    /// Returns the target of the item if it is a reference to another item or member.
    fn dereference_iid(
        &mut self,
        id: ItemId,
        deref_define: bool,
    ) -> Result<(ItemId, Replacements), String> {
        match self.src.definition_of(id).as_ref().unwrap() {
            stage2::Item::Defining { base, .. } => {
                if deref_define {
                    self.dereference_iid(*base, true)
                } else {
                    Ok((id, vec![]))
                }
            }
            stage2::Item::Item(id) => self.dereference_iid(*id, deref_define),
            stage2::Item::Member { base, name } => {
                let (id, mut reps) = self.get_member(*base, name)?;
                let (did, mut dreps) = self.dereference_iid(id, deref_define)?;
                reps.append(&mut dreps);
                Ok((did, reps))
            }
            stage2::Item::Replacing { base, replacements } => {
                let mut deref_base = self.dereference_iid(*base, deref_define)?;
                let mut replacements = self.convert_reps(replacements)?;
                deref_base.1.append(&mut replacements);
                Ok(deref_base)
            }
            _ => Ok((id, vec![])),
        }
    }
}
