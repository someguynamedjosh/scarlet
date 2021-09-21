use crate::stage2::structure::{ItemId, PrimitiveType, PrimitiveValue, Replacements};
use crate::stage3::structure::{self as stage3, Item};
use std::collections::HashMap;
use std::{
    collections::HashSet,
    fmt::{self, Debug, Formatter},
};

pub fn ingest(from: stage3::Environment) -> Result<Environment, String> {
    let mut env = Environment::new(from);
    let mut next_item = ItemId(0);
    while next_item.0 < env.items.len() {
        env.compute_type(next_item)?;
        next_item.0 += 1;
    }
    println!("{:#?}", env);
    let mut next_item = ItemId(0);
    while next_item.0 < env.items.len() {
        env.type_check(next_item)?;
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
    item_reverse_lookup: HashMap<Item, ItemId>,
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
        Self::new(stage3::Environment::new())
    }

    pub fn new(from: stage3::Environment) -> Self {
        let item_reverse_lookup = from
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| (item.clone(), ItemId(index)))
            .collect();
        let items = from
            .items
            .into_iter()
            .map(|i| TypedItem {
                base: i,
                typee: None,
            })
            .collect();
        Self {
            modules: from.modules,
            items,
            item_reverse_lookup,
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

    pub fn insert(&mut self, def: Item) -> ItemId {
        if let Some(existing_id) = self.item_reverse_lookup.get(&def) {
            return *existing_id;
        }
        let id = ItemId(self.items.len());
        self.item_reverse_lookup.insert(def.clone(), id);
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

impl Environment {
    fn resolve_variable(&self, reference: ItemId) -> Result<ItemId, String> {
        assert!(reference.0 < self.items.len());
        let item = &self.items[reference.0];
        match &item.base {
            Item::Defining { base, .. } => {
                let base = *base;
                self.resolve_variable(base)
            }
            Item::FromType { .. } => todo!("nice error"),
            Item::Replacing { base, .. } => {
                let base = *base;
                self.resolve_variable(base)
            }
            Item::GodType
            | Item::InductiveType(..)
            | Item::InductiveValue { .. }
            | Item::PrimitiveType(..)
            | Item::PrimitiveValue(..) => todo!("nice error, not a variable"),
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

    /// Returns true if the two items are defined as the same. This check does
    /// not always return true when this is the case, due to Godel-related math
    /// gremlins.
    fn are_def_equal(&self, left: ItemId, right: ItemId) -> bool {
        if left == right {
            true
        } else {
            // TODO: This is impolite, we could try a little harder than that.
            false
        }
    }

    /// Returns the type of the variable given by the id, assuming the id points to a variable.
    fn get_var_type(&self, var: ItemId) -> ItemId {
        match &self.items[var.0].base {
            Item::Variable { typee, .. } => *typee,
            _ => panic!("{:?} is not a variable", var),
        }
    }

    /// Checks that, if this item is a Replacing item, that it obeys a type check.
    fn type_check(&self, item: ItemId) -> Result<(), String> {
        match &self.items[item.0].base {
            Item::Replacing { replacements, .. } => {
                for (target, val) in replacements {
                    let var_type = self.get_var_type(*target);
                    let val_type = self.items[val.0].typee.unwrap();
                    if !self.are_def_equal(var_type, val_type) {
                        return Err(format!(
                            "(at {:?}) {:?} and {:?} have differing types",
                            item, target, val
                        ));
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    // Collects all variables specified by From items pointed to by the provided ID.
    fn get_from_variables(&mut self, typee: ItemId) -> Result<HashSet<ItemId>, String> {
        Ok(match &self.items[typee.0].base {
            Item::Defining { base: id, .. } => {
                let id = *id;
                self.get_from_variables(id)?
            }
            Item::FromType { base, vars } => {
                let base = *base;
                let vars: HashSet<_> = vars.iter().copied().collect();
                let result = self.get_from_variables(base)?;
                result.union(&vars).copied().collect()
            }
            Item::Replacing { .. } => todo!(),
            _ => HashSet::new(),
        })
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
            Item::InductiveType(..) => self.god_type(),
            Item::InductiveValue { typee, records, .. } => {
                let mut from_vars = HashSet::new();
                let typee = *typee;
                for recorded in records.clone() {
                    let typee = self.compute_type(recorded)?;
                    for from_var in self.get_from_variables(typee)? {
                        from_vars.insert(from_var);
                    }
                }
                self.insert(Item::FromType {
                    base: typee,
                    vars: from_vars.into_iter().collect(),
                })
            }
            Item::PrimitiveType(..) => self.god_type(),
            Item::PrimitiveValue(pv) => match pv {
                PrimitiveValue::I32(..) => self.i32_type(),
            },
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
}
