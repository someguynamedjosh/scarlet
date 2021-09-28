use super::structure::{Environment, ItemDefinition};
use crate::shared::{BuiltinValue, Item, ItemId, Replacements, VarList};

impl Environment {
    pub fn get_or_insert(&mut self, item: Item, defined_in: Option<ItemId>) -> ItemId {
        for (index, this_item) in self.items.iter().enumerate() {
            if this_item.definition == item {
                return ItemId(index);
            }
        }
        let id = ItemId(self.items.len());
        self.items.push(ItemDefinition {
            info_requested: None,
            is_scope: false,
            definition: item,
            defined_in,
            cached_type: None,
        });
        id
    }

    fn get(&self, item: ItemId) -> &ItemDefinition {
        &self.items[item.0]
    }

    fn get_mut(&mut self, item: ItemId) -> &mut ItemDefinition {
        &mut self.items[item.0]
    }

    fn get_item(&self, item: ItemId) -> &Item {
        &self.get(item).definition
    }

    pub fn get_type(&mut self, of: ItemId) -> ItemId {
        if let Some(typee) = self.get(of).cached_type {
            return typee;
        }
        let typee = match self.get_item(of) {
            Item::Any { selff, typee } => {
                let (selff, typee) = (*selff, *typee);
                let typee = self.reduce(typee, &Default::default(), &[]);
                let type_type = self.get_type(typee);
                self.flatten_from_types(&[typee, type_type], &[selff])
            }
            Item::BuiltinValue(val) => {
                let val = *val;
                self.type_of_builtin_value(val)
            }
            Item::Defining { base, .. } => {
                let base = *base;
                self.get_type(base)
            }
            Item::FromType { base, .. } => {
                let base = *base;
                self.get_type(base)
            }
            Item::Variant { typee, .. } => {
                let typee = *typee;
                self.reduce(typee, &Default::default(), &[])
            }
            _ => todo!(),
        };
        self.get_mut(of).cached_type = Some(typee);
        typee
    }

    pub fn elaborate_all_types(&mut self) {
        let mut id = ItemId(0);
        while id.0 < self.items.len() {
            self.get_type(id);
            id.0 += 1;
        }
    }

    fn get_from_vars_and_base(&self, typee: ItemId) -> (VarList, ItemId) {
        match self.get_item(typee) {
            Item::Defining { base, .. } => self.get_from_vars_and_base(*base),
            Item::FromType { base, vars } => {
                let (mut list, base) = self.get_from_vars_and_base(*base);
                list.append(vars);
                (list, base)
            }
            _ => (VarList::new(), typee),
        }
    }

    /// Takes the given types and combines all the variables they specify (if
    /// they are From types) and combines them into a new From type.
    fn flatten_from_types(&mut self, types: &[ItemId], extra_vars: &[ItemId]) -> ItemId {
        assert!(types.len() > 0);
        let mut total_vars = VarList::new();
        let mut total_base = None;
        for typee in types {
            let (vars, base) = self.get_from_vars_and_base(*typee);
            total_vars.append(&vars);
            if total_base.is_none() {
                total_base = Some(base);
            }
        }
        for var in extra_vars {
            total_vars.push(*var);
        }
        let item = Item::FromType {
            base: total_base.unwrap(),
            vars: total_vars,
        };
        let defined_in = self.get(types[0]).defined_in;
        self.get_or_insert(item, defined_in)
    }

    /// Returns the from vars of the item's type.
    fn get_dependencies(&mut self, item: ItemId) -> VarList {
        let item_type = self.get_type(item);
        self.get_from_vars_and_base(item_type).0
    }

    fn reduce(&mut self, item: ItemId, reps: &Replacements, visited: &[ItemId]) -> ItemId {
        if visited.contains(&item) {
            todo!()
        }
        let visited: Vec<_> = visited
            .iter()
            .copied()
            .chain(std::iter::once(item))
            .collect();
        let defined_in = self.get(item).defined_in;
        match self.get_item(item) {
            Item::Any { selff, .. } => reps.applied_to(*selff),
            Item::BuiltinValue(..) => item,
            Item::FromType { base, vars } => {
                let (base, values) = (*base, vars.clone());
                let mut vars_before_reps = VarList::new();
                for val in values {
                    vars_before_reps.append(&self.get_dependencies(val));
                }
                let mut vars_after_reps = VarList::new();
                for var in vars_before_reps {
                    let repped = reps.applied_to(var);
                    vars_after_reps.append(&self.get_dependencies(repped));
                }
                let rbase = self.reduce(base, reps, &visited);
                let (base, vars) = (rbase, vars_after_reps);
                let item = Item::FromType { base, vars };
                self.get_or_insert(item, defined_in)
            }
            Item::Variant { selff, typee } => {
                let (selff, typee) = (*selff, *typee);
                let item = Item::Variant {
                    selff,
                    typee: self.reduce(typee, reps, &visited),
                };
                self.get_or_insert(item, defined_in)
            }
            _ => todo!(),
        }
    }

    fn type_of_builtin_value(&mut self, val: BuiltinValue) -> ItemId {
        // The types of builtin values are themselves builtin values.
        let type_val = match val {
            BuiltinValue::PrimaryType | BuiltinValue::BoolType | BuiltinValue::I32Type => {
                BuiltinValue::PrimaryType
            }
            BuiltinValue::Bool(..) => BuiltinValue::BoolType,
            BuiltinValue::I32(..) => BuiltinValue::I32Type,
        };
        let item = Item::BuiltinValue(type_val);
        self.get_or_insert(item, None)
    }
}
