use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    fmt::Debug,
    ops::Index,
};

use crate::{
    definitions::{
        builtin::DBuiltin, compound_type::DCompoundType, identifier::DIdentifier,
        member_access::DMemberAccess, parameter::DParameter, struct_literal::DStructLiteral,
    },
    diagnostic::Diagnostic,
    item::query::{Query, QueryContext, RootQuery},
};

thread_local! {
    pub static ENV: RefCell<Environment<Def0>> = RefCell::new(Environment::new());
    pub static FLAG: Cell<bool> = Cell::new(false);
}

/// This struct guarantees certain parts of the code remain internal to the
/// library without having to put them in the same module.
pub(crate) struct OnlyConstructedByEnvironment(());

macro_rules! def_enum {
    ($Name:ident { $($Variant:ident),* }) => {
        #[derive(Clone, Debug)]
        pub enum $Name {
            $($Variant($Variant)),*
        }

        $(
            impl From<$Variant> for $Name {
                fn from(v: $Variant) -> Self {
                    Self::$Variant(v)
                }
            }
        )*
    };
}

def_enum!(Def0 {
    DBuiltin,
    DCompoundType,
    DIdentifier,
    DMemberAccess,
    DParameter,
    DStructLiteral
});

pub type Env0 = Environment<Def0>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(usize);

impl Debug for ItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "I#{}", self.0)
    }
}

#[derive(Clone)]
pub struct Environment<Def> {
    language_items: HashMap<String, ItemId>,
    root: ItemId,
    all_items: Vec<Option<Def>>,
}

impl<Def: Debug> Debug for Environment<Def> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Environment")?;
        writeln!(f)?;
        writeln!(f, "Language Items:")?;
        for (key, value) in &self.language_items {
            writeln!(f, "{:?} => {:#?}", key, value)?;
        }
        writeln!(f)?;
        writeln!(f, "Items:")?;
        for (key, value) in self.all_items.iter().enumerate() {
            write!(f, "{} => ", key)?;
            if let Some(item) = value {
                writeln!(f, "{:#?}", item)?;
            } else {
                writeln!(f, "Undefined")?;
            }
        }
        Ok(())
    }
}

impl<Def> Index<ItemId> for Environment<Def> {
    type Output = Def;

    fn index(&self, index: ItemId) -> &Self::Output {
        if let Some(item) = &self.all_items[index.0] {
            item
        } else {
            panic!("Item associated with {:?} is undefined.", index)
        }
    }
}

impl<Def: From<DStructLiteral>> Environment<Def> {
    pub(crate) fn new() -> Self {
        let root = ItemId(0);
        let mut this = Self {
            language_items: HashMap::new(),
            root,
            all_items: vec![None],
        };
        this.define_item(root, DStructLiteral::new_module(vec![]));
        this
    }
}

impl<Def> Environment<Def> {
    pub fn new_item(&mut self) -> ItemId {
        let id = self.all_items.len();
        self.all_items.push(None);
        ItemId(id)
    }

    pub fn new_defined_item(&mut self, definition: impl Into<Def>) -> ItemId {
        let id = self.new_item();
        self.define_item(id, definition);
        id
    }

    pub fn define_item(&mut self, item: ItemId, definition: impl Into<Def>) {
        let item = &mut self.all_items[item.0];
        assert!(item.is_none());
        *item = Some(definition.into())
    }

    pub fn define_language_item(
        &mut self,
        name: &str,
        definition: ItemId,
    ) -> Result<(), Diagnostic> {
        if self.language_items.contains_key(name) {
            Err(Diagnostic::new().with_text_error(format!(
                "Language item \"{}\" defined multiple times.",
                name
            )))
        } else {
            self.language_items.insert(name.to_owned(), definition);
            Ok(())
        }
    }

    pub fn get_language_item(&self, name: &str) -> Result<&ItemId, Diagnostic> {
        self.language_items.get(name).ok_or_else(|| {
            Diagnostic::new()
                .with_text_error(format!("The language item \"{}\" is not defined.", name))
        })
    }

    pub fn get_root(&self) -> &ItemId {
        &self.root
    }

    pub fn root_query() -> QueryContext<RootQuery> {
        QueryContext::root(OnlyConstructedByEnvironment(()))
    }
}
