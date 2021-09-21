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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum DereferencedItem {
    Stage2Item(ItemId),
    Replacing {
        base: Box<DereferencedItem>,
        replacements: Replacements,
        unlabeled_replacements: Vec<ItemId>,
    },
}

impl DereferencedItem {
    pub fn id(&self) -> ItemId {
        match self {
            Self::Stage2Item(id) => *id,
            Self::Replacing { base, .. } => base.id(),
        }
    }

    pub fn with_base(&self, new_base: DereferencedItem) -> Self {
        match self {
            Self::Stage2Item(..) => new_base,
            Self::Replacing {
                base,
                replacements,
                unlabeled_replacements,
            } => Self::Replacing {
                base: Box::new(base.with_base(new_base)),
                replacements: replacements.clone(),
                unlabeled_replacements: unlabeled_replacements.clone(),
            },
        }
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
            stage2::Item::Replacing {
                base,
                replacements,
                unlabeled_replacements,
            } => Item::Replacing {
                base: self.full_convert_iid(*base)?,
                replacements: self.convert_reps(replacements)?,
                unlabeled_replacements: self.convert_iids(unlabeled_replacements)?,
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

    fn convert_dereffed(&mut self, item: DereferencedItem) -> ItemId {
        match item {
            DereferencedItem::Stage2Item(id) => *self.id_map.get(&id).unwrap(),
            DereferencedItem::Replacing {
                base,
                replacements,
                unlabeled_replacements,
            } => {
                let base = self.convert_dereffed(*base);
                self.insert_extra_item(Item::Replacing {
                    base,
                    replacements,
                    unlabeled_replacements,
                })
            }
        }
    }

    /// Applies dereferencing and id_map to the provided item id.
    fn convert_iid(&mut self, id: ItemId, deref_define: bool) -> Result<ItemId, String> {
        let dereffed = self.dereference_iid(id, deref_define)?;
        Ok(self.convert_dereffed(dereffed))
    }

    fn get_member(
        &mut self,
        base: ItemId,
        name: &String,
        deref_define: bool,
    ) -> Result<DereferencedItem, String> {
        let og_base = self.dereference_iid(base, false)?;
        match self.src.definition_of(og_base.id()).as_ref().unwrap() {
            stage2::Item::Defining {
                base: def_base,
                definitions,
            } => {
                if let Ok(member) = self.get_member(*def_base, name, deref_define) {
                    return Ok(member);
                }
                for (cname, cdef) in definitions {
                    if cname == name {
                        let dereffed_definition = self.dereference_iid(*cdef, deref_define)?;
                        return Ok(og_base.with_base(dereffed_definition));
                    }
                }
                Err(format!("{:?} has no member named {}", og_base, name))
            }
            stage2::Item::Item(..) | stage2::Item::Member { .. } => unreachable!(),
            stage2::Item::Replacing { .. } => unreachable!("{:?} {:?}", og_base, base),
            _ => Err(format!("{:?} has no members", og_base)),
        }
    }

    /// Returns the target of the item if it is a reference to another item or member.
    fn dereference_iid(
        &mut self,
        id: ItemId,
        deref_define: bool,
    ) -> Result<DereferencedItem, String> {
        match self.src.definition_of(id).as_ref().unwrap() {
            stage2::Item::Defining { base, .. } => {
                if deref_define {
                    self.dereference_iid(*base, true)
                } else {
                    Ok(DereferencedItem::Stage2Item(id))
                }
            }
            stage2::Item::Item(id) => self.dereference_iid(*id, deref_define),
            stage2::Item::Member { base, name } => self.get_member(*base, name, deref_define),
            stage2::Item::Replacing {
                base,
                replacements,
                unlabeled_replacements,
            } => {
                let deref_base = self.dereference_iid(*base, deref_define)?;
                let replacements = self.convert_reps(replacements)?;
                let unlabeled_replacements = self.convert_iids(unlabeled_replacements)?;
                Ok(DereferencedItem::Replacing {
                    base: Box::new(deref_base),
                    replacements,
                    unlabeled_replacements,
                })
            }
            _ => Ok(DereferencedItem::Stage2Item(id)),
        }
    }
}
