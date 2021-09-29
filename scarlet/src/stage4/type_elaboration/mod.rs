use super::structure::{Environment, ItemDefinition};
use crate::shared::{
    BuiltinOperation, BuiltinValue, IntegerMathOperation, Item, ItemId, Replacements, VarList,
};

impl Environment {
    pub fn get_or_insert(&mut self, item: Item, defined_in: Option<ItemId>) -> ItemId {
        for (index, this_item) in self.items.iter().enumerate() {
            if this_item.definition == item {
                return ItemId(index);
            }
        }
        let id = ItemId(self.items.len());
        self.items.push(ItemDefinition {
            info_requested_in: vec![],
            is_scope: false,
            definition: item,
            defined_in,
            cached_type: None,
            cached_reduction: None,
        });
        id
    }

    pub fn get_or_insert_def(&mut self, def: ItemDefinition) -> ItemId {
        for (index, this_item) in self.items.iter().enumerate() {
            if this_item == &def {
                return ItemId(index);
            }
        }
        let id = ItemId(self.items.len());
        self.items.push(def);
        id
    }

    pub fn get_or_insert_primary_type(&mut self) -> ItemId {
        self.get_or_insert(Item::BuiltinValue(BuiltinValue::OriginType), None)
    }

    pub fn get_or_insert_bool_type(&mut self) -> ItemId {
        self.get_or_insert(Item::BuiltinValue(BuiltinValue::BoolType), None)
    }

    pub fn get_or_insert_i32_type(&mut self) -> ItemId {
        self.get_or_insert(Item::BuiltinValue(BuiltinValue::I32Type), None)
    }

    pub fn get(&self, item: ItemId) -> &ItemDefinition {
        &self.items[item.0]
    }

    pub fn get_mut(&mut self, item: ItemId) -> &mut ItemDefinition {
        &mut self.items[item.0]
    }

    pub fn get_item(&self, item: ItemId) -> &Item {
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
            Item::BuiltinOperation(op) => {
                let op = *op;
                let inputs = op.inputs();
                let bt = self.builtin_op_base_type(op);
                let mut types = vec![bt];
                for input in inputs {
                    types.push(self.get_type(input));
                }
                self.flatten_from_types(&types, &[])
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
            Item::Pick { .. } => todo!(),
            Item::Replacing { base, replacements } => {
                let (base, reps) = (*base, replacements.clone());
                let base_type = self.get_type(base);
                self.reduce(base_type, &reps, &[])
            }
            Item::TypeIs { typee, .. } | Item::Variant { typee, .. } => {
                let typee = *typee;
                let rtype = self.reduce(typee, &Default::default(), &[]);
                let deps = self.get_dependencies(rtype);
                if deps.len() == 0 {
                    rtype
                } else {
                    let item = Item::FromType {
                        base: rtype,
                        vals: deps,
                    };
                    let defined_in = self.get(typee).defined_in;
                    self.get_or_insert(item, defined_in)
                }
            }
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

    fn builtin_op_base_type(&mut self, op: BuiltinOperation) -> ItemId {
        match op {
            BuiltinOperation::I32Math(..) => self.get_or_insert_i32_type(),
            BuiltinOperation::AreSameVariant { .. } => self.get_or_insert_bool_type(),
            BuiltinOperation::Reinterpret { new_type, .. } => new_type,
        }
    }

    fn get_from_vars_and_base(&self, typee: ItemId) -> (VarList, ItemId) {
        match self.get_item(typee) {
            Item::Defining { base, .. } => self.get_from_vars_and_base(*base),
            Item::FromType { base, vals: vars } => {
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
        if total_vars.len() > 0 {
            let item = Item::FromType {
                base: total_base.unwrap(),
                vals: total_vars,
            };
            let defined_in = self.get(types[0]).defined_in;
            self.get_or_insert(item, defined_in)
        } else {
            total_base.unwrap()
        }
    }

    /// Returns the from vars of the item's type.
    fn get_dependencies(&mut self, item: ItemId) -> VarList {
        let item_type = self.get_type(item);
        self.get_from_vars_and_base(item_type).0
    }

    pub fn reduce_all_items(&mut self) {
        let mut id = ItemId(0);
        while id.0 < self.items.len() {
            let new = self.reduce(id, &Default::default(), &[]);
            if id != new {
                self.get_type(new);
            }
            id.0 += 1;
        }
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
        let reduced = match self.get_item(item) {
            Item::Any { selff, typee } => {
                let new_val = reps.applied_to(*selff);
                if new_val != *selff {
                    new_val
                } else {
                    let (selff, typee) = (*selff, *typee);
                    let new_type = self.reduce(typee, reps, &visited);
                    if new_type == typee {
                        item
                    } else {
                        let typee = new_type;
                        let item = Item::Any { selff, typee };
                        self.get_or_insert(item, defined_in)
                    }
                }
            }
            Item::BuiltinOperation(op) => {
                let op = *op;
                let mut rinputs = Vec::new();
                for input in op.inputs() {
                    rinputs.push(self.reduce(input, reps, &visited));
                }
                if let Some(reduced) = self.reduce_builtin_operation(defined_in, op, &rinputs) {
                    reduced
                } else {
                    item
                }
            }
            Item::BuiltinValue(..) => item,
            Item::Defining { base, .. } => {
                let base = *base;
                self.reduce(base, reps, &visited)
            }
            Item::FromType { base, vals } => {
                let (base, values) = (*base, vals.clone());
                let mut new_vals = VarList::new();
                for val in values {
                    let rval = self.reduce(val, reps, &visited);
                    let rdeps = self.get_dependencies(rval);
                    new_vals.append(&rdeps);
                }
                let vals = new_vals;
                let base = self.reduce(base, reps, &visited);
                if vals.len() == 0 {
                    base
                } else {
                    let item = Item::FromType { base, vals };
                    self.get_or_insert(item, defined_in)
                }
            }
            Item::Pick { .. } => todo!(),
            Item::Replacing { base, replacements } => {
                let base = *base;
                let mut new_reps = reps.clone();
                for rep in replacements.clone() {
                    let new_val = self.reduce(rep.1, reps, &visited);
                    new_reps.insert_or_replace((rep.0, new_val));
                }
                let rbase = self.reduce(base, &new_reps, &visited);
                let rdeps = self.get_dependencies(rbase);
                // Replacements of variables the expression is still dependant on.
                let mut remaining_reps = Replacements::new();
                for rep in new_reps {
                    if rdeps.contains(rep.0) {
                        remaining_reps.insert_no_replace(rep);
                    }
                }
                if remaining_reps.len() == 0 {
                    rbase
                } else {
                    let item = Item::Replacing {
                        base: rbase,
                        replacements: remaining_reps,
                    };
                    self.get_or_insert(item, defined_in)
                }
            }
            Item::TypeIs { base, .. } => {
                let base = *base;
                self.reduce(base, reps, &visited)
            }
            Item::Variant { selff, typee } => item,
        };
        if reps.len() == 0 {
            self.get_mut(item).cached_reduction = Some(reduced);
        }
        reduced
    }

    fn get_builtin_value(&self, from: ItemId) -> Option<&BuiltinValue> {
        match self.get_item(from) {
            Item::Defining { base, .. } => {
                let base = *base;
                self.get_builtin_value(base)
            }
            Item::BuiltinValue(val) => Some(val),
            _ => None,
        }
    }

    fn map_op_inputs<T>(
        &self,
        mapper: &impl Fn(&BuiltinValue) -> Option<T>,
        inputs: &[ItemId],
    ) -> Option<Vec<T>> {
        inputs
            .iter()
            .map(|i| self.get_builtin_value(*i).map(mapper).flatten())
            .collect()
    }

    fn reduce_builtin_operation(
        &mut self,
        defined_in: Option<ItemId>,
        op: BuiltinOperation,
        inputs: &[ItemId],
    ) -> Option<ItemId> {
        match op {
            BuiltinOperation::AreSameVariant { .. } => todo!(),
            BuiltinOperation::I32Math(op) => {
                let inputs = self.map_op_inputs(&BuiltinValue::as_i32, inputs)?;
                let val = match op {
                    IntegerMathOperation::Sum(..) => {
                        assert_eq!(inputs.len(), 2);
                        inputs[0] + inputs[1]
                    }
                    IntegerMathOperation::Difference(..) => {
                        assert_eq!(inputs.len(), 2);
                        inputs[0] - inputs[1]
                    }
                };
                let item = Item::BuiltinValue(BuiltinValue::I32(val));
                Some(self.get_or_insert(item, defined_in))
            }
            BuiltinOperation::Reinterpret { .. } => {
                if self.get_dependencies(inputs[0]).len() > 0
                    || self.get_dependencies(inputs[1]).len() > 0
                    || self.get_dependencies(inputs[2]).len() > 0
                    || self.get_from_vars_and_base(inputs[1]).0.len()
                        != self.get_dependencies(inputs[3]).len()
                {
                    return None;
                }
                let original_def = self.get(inputs[3]).clone();
                let item_def = ItemDefinition {
                    cached_type: Some(inputs[2]),
                    ..original_def
                };
                Some(self.get_or_insert_def(item_def))
            }
        }
    }

    fn type_of_builtin_value(&mut self, val: BuiltinValue) -> ItemId {
        match val {
            BuiltinValue::OriginType | BuiltinValue::BoolType | BuiltinValue::I32Type => {
                self.get_or_insert_primary_type()
            }
            BuiltinValue::Bool(..) => self.get_or_insert_bool_type(),
            BuiltinValue::I32(..) => self.get_or_insert_i32_type(),
        }
    }
}
