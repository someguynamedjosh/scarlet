use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};

use crate::{
    stage2::structure::ItemId,
    stage3::structure::{self as stage3, Item},
};

#[derive(Clone, PartialEq)]
pub struct TypedItem {
    pub(super) base: Item,
    pub(super) typee: Option<ItemId>,
}

#[derive(Clone, PartialEq)]
pub struct Environment {
    pub modules: Vec<ItemId>,
    pub(super) items: Vec<TypedItem>,
    pub(super) item_reverse_lookup: HashMap<Item, ItemId>,
}

fn indented(source: &str) -> String { source.replace("\n", "\n    ") }

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
    pub fn _new_empty() -> Self { Self::new(stage3::Environment::new()) }

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

    pub(super) fn _iter(&self) -> impl Iterator<Item = (ItemId, &Item, &Option<ItemId>)> {
        self.items
            .iter()
            .enumerate()
            .map(|(index, val)| (ItemId(index), &val.base, &val.typee))
    }

    pub(super) fn _iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (ItemId, &mut Item, &mut Option<ItemId>)> {
        self.items
            .iter_mut()
            .enumerate()
            .map(|(index, val)| (ItemId(index), &mut val.base, &mut val.typee))
    }

    pub(super) fn insert(&mut self, def: Item) -> ItemId {
        if let Some(existing_id) = self.item_reverse_lookup.get(&def) {
            return *existing_id;
        }
        let id = ItemId(self.items.len());
        println!("inserted {:?} {:#?}", id, def);
        self.item_reverse_lookup.insert(def.clone(), id);
        self.items.push(TypedItem {
            base: def,
            typee: None,
        });
        id
    }

    pub(super) fn insert_with_type(&mut self, def: Item, typee: ItemId) -> ItemId {
        if let Some(existing_id) = self.item_reverse_lookup.get(&def) {
            return *existing_id;
        }
        let id = ItemId(self.items.len());
        self.item_reverse_lookup.insert(def.clone(), id);
        self.items.push(TypedItem {
            base: def,
            typee: Some(typee),
        });
        id
    }

    pub(super) fn set_type(&mut self, item: ItemId, typee: ItemId) {
        assert!(item.0 < self.items.len());
        self.items[item.0].typee = Some(typee)
    }
}
