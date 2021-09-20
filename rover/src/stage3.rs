use crate::stage2::{self, Item, ItemId, PrimitiveType, PrimitiveValue, Replacements};
use std::fmt::{self, Debug, Formatter};

pub fn ingest(from: stage2::Environment) -> Result<Environment, String> {
    let mut env = Environment::new(from);
    let mut next_item = ItemId(0);
    while next_item.0 < env.items.len() {
        env.compute_type(next_item)?;
        next_item.0 += 1;
    }
    Ok(env)
}

#[derive(Clone, PartialEq)]
struct TypedItem {
    base: Item,
    typee: Option<ItemId>,
}

#[derive(Clone, PartialEq)]
pub struct Environment {
    pub modules: Vec<ItemId>,
    items: Vec<TypedItem>,
}

fn indented(source: &str) -> String {
    source.replace("\n", "\n    ")
}

impl Debug for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Environment [")?;
        for (index, item) in self.items.iter().enumerate() {
            if f.alternate() {
                write!(f, "\n\n    ")?;
            }
            write!(f, "{:?} is ", ItemId(index))?;
            if f.alternate() {
                let text = format!("{:#?}", item.base);
                write!(f, "{}\n    ", indented(&text))?;
            } else {
                write!(f, "{:?} ", item.base)?;
            }
            write!(f, "type_is{{ ")?;
            match &item.typee {
                Some(item) => write!(f, "{:?}", item)?,
                None => write!(f, "?")?,
            }
            write!(f, " }}")?;
        }
        if f.alternate() {
            write!(f, "\n")?;
        }
        write!(f, "]")
    }
}

impl Environment {
    pub fn new_empty() -> Self {
        Self::new(stage2::Environment::new())
    }

    pub fn new(from: stage2::Environment) -> Self {
        Self {
            modules: from.modules,
            items: from
                .items
                .into_iter()
                .map(|i| TypedItem {
                    base: i.unwrap(),
                    typee: None,
                })
                .collect(),
        }
    }

    fn existing_item(&self, def: &Item) -> Option<ItemId> {
        for (index, item) in self.items.iter().enumerate() {
            if &item.base == def {
                return Some(ItemId(index));
            }
        }
        None
    }

    fn god_type(&self) -> ItemId {
        self.existing_item(&Item::GodType).unwrap()
    }

    fn i32_type(&self) -> ItemId {
        self.existing_item(&Item::PrimitiveType(PrimitiveType::I32))
            .unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = (ItemId, &Item, &Option<ItemId>)> {
        self.items
            .iter()
            .enumerate()
            .map(|(index, val)| (ItemId(index), &val.base, &val.typee))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (ItemId, &mut Item, &mut Option<ItemId>)> {
        self.items
            .iter_mut()
            .enumerate()
            .map(|(index, val)| (ItemId(index), &mut val.base, &mut val.typee))
    }

    pub fn get_member(&mut self, base: ItemId, name: &String) -> Result<ItemId, String> {
        let base_item = &self.items[base.0];
        let member_name = name;
        match &base_item.base {
            Item::Defining { base, definitions } => {
                let mut candidate = None;
                for (name, val) in definitions {
                    if name == member_name {
                        candidate = Some(*val)
                    }
                }
                let base = *base;
                if let Ok(res) = self.get_member(base, name) {
                    return Ok(res);
                } else if let Some(val) = candidate {
                    return Ok(val);
                }
            }
            Item::FromType { base, .. } => {
                let base = *base;
                return self.get_member(base, member_name);
            }
            Item::Item(base) => {
                let base = *base;
                return self.get_member(base, member_name);
            }
            Item::Member { base, name } => {
                let base = *base;
                let name = name.clone();
                let that_member = self.get_member(base, &name)?;
                return self.get_member(that_member, member_name);
            }
            Item::Public(..) => todo!(),
            Item::Replacing { .. } => todo!(),
            Item::GodType
            | Item::InductiveType(..)
            | Item::InductiveValue { .. }
            | Item::PrimitiveType(..)
            | Item::PrimitiveValue(..)
            | Item::Variable { .. } => (),
        }
        // Don't todoify this, some code relies on catching errors.
        Err(format!("failed to find a member named ident{{{}}}", name))
    }

    fn resolve_variable(&mut self, reference: ItemId) -> Result<ItemId, String> {
        assert!(reference.0 < self.items.len());
        let item = &self.items[reference.0];
        match &item.base {
            Item::Defining { base, .. } => {
                let base = *base;
                self.resolve_variable(base)
            }
            Item::FromType { .. } => todo!("nice error"),
            Item::InductiveValue { .. } => todo!("nice error"),
            Item::Item(id) => {
                let id = *id;
                self.resolve_variable(id)
            }
            Item::Member { base, name } => {
                let base = *base;
                let name = name.clone();
                let item = self.get_member(base, &name)?;
                self.resolve_variable(item)
            }
            Item::Public(id) => {
                let id = *id;
                self.resolve_variable(id)
            }
            Item::Replacing { base, .. } => {
                let base = *base;
                self.resolve_variable(base)
            }
            Item::GodType
            | Item::InductiveType(..)
            | Item::InductiveValue { .. }
            | Item::PrimitiveType(..)
            | Item::PrimitiveValue(..) => todo!("nice error"),
            Item::Variable { selff, .. } => Ok(*selff),
        }
    }

    fn compute_type_after_replacing(
        &mut self,
        base: ItemId,
        replacements: Replacements,
    ) -> Result<ItemId, String> {
        let unreplaced_type = self.compute_type(base)?;
        let mut ids_to_replace = Vec::new();
        for (id, _) in replacements {
            ids_to_replace.push(self.resolve_variable(id)?)
        }
        let def = &self.items[unreplaced_type.0].base;
        let res = match def {
            Item::FromType { base, vars } => {
                let mut vars_after_reps = vars.clone();
                for index in (0..vars_after_reps.len()).rev() {
                    if ids_to_replace
                        .iter()
                        .any(|id| *id == vars_after_reps[index])
                    {
                        vars_after_reps.remove(index);
                    }
                }
                if vars_after_reps.len() == 0 {
                    *base
                } else if &vars_after_reps == vars {
                    unreplaced_type
                } else {
                    let base = *base;
                    self.insert(Item::FromType {
                        base,
                        vars: vars_after_reps,
                    })
                }
            }
            _ => unreplaced_type,
        };
        Ok(res)
    }

    fn compute_type(&mut self, of: ItemId) -> Result<ItemId, String> {
        assert!(of.0 < self.items.len());
        let item = &self.items[of.0];
        let typee = match &item.base {
            Item::Defining { base, .. } => {
                let base = *base;
                self.compute_type(base)?
            }
            // TODO: This is not always correct.
            Item::FromType { .. } => self.god_type(),
            Item::GodType { .. } => self.god_type(),
            // TODO: This is not always correct. Need to finalize how inductive
            // types can depend on vars.
            Item::InductiveType(id) => self.god_type(),
            Item::InductiveValue { typee, .. } => *typee,
            Item::Item(id) => {
                let id = *id;
                self.compute_type(id)?;
                self.items[id.0].typee.unwrap()
            }
            Item::Member { base, name } => {
                let base = *base;
                let name = name.clone();
                let member = self.get_member(base, &name)?;
                self.compute_type(member)?
            }
            Item::PrimitiveType(..) => self.god_type(),
            Item::PrimitiveValue(pv) => match pv {
                PrimitiveValue::I32(..) => self.i32_type(),
            },
            Item::Public(..) => todo!(),
            Item::Replacing { base, replacements } => {
                let base = *base;
                let replacements = replacements.clone();
                self.compute_type_after_replacing(base, replacements)?
            }
            Item::Variable { typee, selff } => {
                let base = *typee;
                let vars = vec![*selff];
                self.insert(Item::FromType { base, vars })
            }
        };
        self.set_type(of, typee);
        Ok(typee)
    }

    pub fn insert(&mut self, def: Item) -> ItemId {
        let id = ItemId(self.items.len());
        self.items.push(TypedItem {
            base: def,
            typee: None,
        });
        id
    }

    pub fn set_type(&mut self, item: ItemId, typee: ItemId) {
        assert!(item.0 < self.items.len());
        self.items[item.0].typee = Some(typee)
    }
}
