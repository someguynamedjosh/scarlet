use crate::util::indented;
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    I32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PrimitiveValue {
    I32(i32),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub(crate) usize);

impl Debug for ItemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "id{{{}}}", self.0)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Environment {
    pub modules: Vec<ItemId>,
    pub(crate) items: Vec<Option<Item>>,
}

impl Debug for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Environment [")?;
        for (index, item) in self.items.iter().enumerate() {
            if f.alternate() {
                write!(f, "\n\n    ")?;
            }
            write!(f, "{:?} is ", ItemId(index))?;
            match item {
                Some(item) => {
                    if f.alternate() {
                        let text = format!("{:#?}", item);
                        write!(f, "{},", indented(&text[..]))?;
                    } else {
                        write!(f, "{:?}", item)?;
                    }
                }
                None => write!(f, "None,")?,
            }
        }
        if f.alternate() {
            write!(f, "\n")?;
        }
        write!(f, "]")
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (ItemId, &Option<Item>)> {
        self.items
            .iter()
            .enumerate()
            .map(|(index, val)| (ItemId(index), val))
    }

    pub fn mark_as_module(&mut self, item: ItemId) {
        self.modules.push(item)
    }

    pub fn next_id(&mut self) -> ItemId {
        let id = ItemId(self.items.len());
        self.items.push(None);
        id
    }

    pub fn define(&mut self, item: ItemId, definition: Item) {
        assert!(item.0 < self.items.len());
        self.items[item.0] = Some(definition)
    }

    pub fn definition_of(&self, item: ItemId) -> &Option<Item> {
        assert!(item.0 < self.items.len());
        &self.items[item.0]
    }
}

pub type Definitions = Vec<(String, ItemId)>;
pub type Replacements = Vec<(ItemId, ItemId)>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum IntegerMathOperation {
    Add(ItemId, ItemId),
    Subtract(ItemId, ItemId),
    // Multiply(ItemId, ItemId),
    // IntegerDivide(ItemId, ItemId),
    // Modulo(ItemId, ItemId),
    // Negate(ItemId),
}

impl Debug for IntegerMathOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add(l, r) => write!(f, "add[{:?} {:?}]", l, r),
            Self::Subtract(l, r) => write!(f, "subtract[{:?} {:?}]", l, r),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveOperation {
    I32Math(IntegerMathOperation),
}

impl Debug for PrimitiveOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::I32Math(op) => write!(f, "Integer32::{:?}", op),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Defining {
        base: ItemId,
        definitions: Definitions,
    },
    FromType {
        base: ItemId,
        vars: Vec<ItemId>,
    },
    GodType,
    InductiveType(ItemId),
    InductiveValue {
        typee: ItemId,
        variant_name: String,
        records: Vec<ItemId>,
    },
    Item(ItemId),
    Member {
        base: ItemId,
        name: String,
    },
    PrimitiveOperation(PrimitiveOperation),
    PrimitiveType(PrimitiveType),
    PrimitiveValue(PrimitiveValue),
    Replacing {
        base: ItemId,
        replacements: Replacements,
    },
    Variable {
        selff: ItemId,
        typee: ItemId,
    },
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Defining { base, definitions } => {
                let gap = if f.alternate() { "\n" } else { "" };
                write!(f, "{:?} {}defining{{", base, gap)?;
                for (name, def) in definitions {
                    if f.alternate() {
                        write!(f, "\n    ")?;
                    }
                    write!(f, "{} is {:?} ", name, def)?;
                }
                write!(f, "{}}}", gap)
            }
            Self::FromType { base, vars } => {
                write!(f, "{:?} From{{", base)?;
                if vars.len() > 0 {
                    write!(f, "{:?}", vars[0])?;
                }
                for var in &vars[1..] {
                    write!(f, " {:?}", var)?;
                }
                write!(f, "}}")
            }
            Self::GodType => write!(f, "TYPE"),
            Self::InductiveType(id) => write!(f, "InductiveType({:?})", id),
            Self::InductiveValue {
                typee,
                variant_name,
                records,
            } => {
                write!(f, "inductive_value {:?}::{}[", typee, variant_name)?;
                for record in records {
                    if f.alternate() {
                        write!(f, "\n    ")?;
                    }
                    write!(f, "{:?}, ", record)?;
                }
                if f.alternate() {
                    write!(f, "\n")?;
                }
                write!(f, "]")
            }
            Self::Item(id) => write!(f, "{:?}", id),
            Self::Member { base, name } => write!(f, "{:?}::{}", base, name),
            Self::PrimitiveOperation(po) => write!(f, "{:?}", po),
            Self::PrimitiveType(pt) => write!(f, "{:?}", pt),
            Self::PrimitiveValue(pv) => write!(f, "{:?}", pv),
            Self::Replacing { base, replacements } => {
                let gap = if f.alternate() { "\n" } else { "" };
                write!(f, "{:?} {}replacing{{", base, gap)?;
                for (target, value) in replacements {
                    if f.alternate() {
                        write!(f, "\n    ")?;
                    }
                    write!(f, "{:?} with {:?} ", target, value)?;
                }
                write!(f, "{}}}", gap)
            }
            Self::Variable { selff, typee } => write!(f, "any{{{:?}}} at {:?}", typee, selff),
        }
    }
}
