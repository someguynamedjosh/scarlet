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
        substitution::DUnresolvedSubstitution,
    },
    diagnostic::{Diagnostic, Position},
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
    DStructLiteral,
    DUnresolvedSubstitution
});

pub type Env0 = Environment<Def0>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(usize);

impl Debug for ItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "I#{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct ItemMetadata {
    pub parent: Option<ItemId>,
    pub position: Option<Position>,
}

impl ItemMetadata {
    pub fn new() -> Self {
        Self {
            parent: None,
            position: None,
        }
    }
}

#[derive(Clone)]
pub struct Environment<Def> {
    language_items: HashMap<String, ItemId>,
    root: ItemId,
    all_items: Vec<(Option<Def>, ItemMetadata)>,
}

impl<Def: Debug> Debug for Environment<Def> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Environment (Root {:?})", self.root)?;
        writeln!(f)?;
        writeln!(f, "Language Items:")?;
        for (key, value) in &self.language_items {
            writeln!(f, "{:?} => {:#?}", key, value)?;
        }
        writeln!(f)?;
        writeln!(f, "Items:")?;
        for (key, (item, meta)) in self.all_items.iter().enumerate() {
            write!(f, "I#{}", key)?;
            if let Some(parent) = meta.parent {
                write!(f, " (Child of {:?})", parent)?;
            }
            if let Some(position) = meta.position {
                write!(f, " ({})", position)?;
            }
            writeln!(f)?;
            if let Some(item) = item {
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
        if let Some(item) = &self.all_items[index.0].0 {
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
            all_items: vec![(None, ItemMetadata::new())],
        };
        this.define_item(root, DStructLiteral::new_module(vec![]));
        this
    }
}

impl<Def> Environment<Def> {
    pub fn new_item(&mut self) -> ItemId {
        let id = self.all_items.len();
        self.all_items.push((None, ItemMetadata::new()));
        ItemId(id)
    }

    pub fn new_defined_item(&mut self, definition: impl Into<Def>) -> ItemId {
        let id = self.new_item();
        self.define_item(id, definition);
        id
    }

    pub fn define_item(&mut self, item: ItemId, definition: impl Into<Def>) {
        let item = &mut self.all_items[item.0].0;
        assert!(item.is_none());
        *item = Some(definition.into())
    }

    pub fn set_position(&mut self, item: ItemId, position: Position) {
        self.all_items[item.0].1.position = Some(position);
    }

    pub fn position(&self, item: ItemId) -> Option<Position> {
        self.all_items[item.0].1.position
    }

    fn set_parent(&mut self, item: ItemId, parent: ItemId) {
        self.all_items[item.0].1.parent = Some(parent);
    }

    pub fn parent(&self, item: ItemId) -> Option<ItemId> {
        self.all_items[item.0].1.parent
    }

    pub fn assert_all_defined(&self) {
        for (index, (def, _)) in self.all_items.iter().enumerate() {
            assert!(
                def.is_some(),
                "Item {} should be defined, but isn't.",
                index
            );
        }
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

    pub fn set_root(&mut self, root: ItemId) {
        self.root = root
    }

    pub fn root(&self) -> ItemId {
        self.root
    }

    pub fn root_query() -> QueryContext<RootQuery> {
        QueryContext::root(OnlyConstructedByEnvironment(()))
    }
}

impl Environment<Def0> {
    pub fn compute_parents(&mut self) {
        self.propogate_parent(self.root)
    }

    fn set_parent_and_propogate(&mut self, child: ItemId, parent: ItemId) {
        self.set_parent(child, parent);
        self.propogate_parent(child);
    }

    fn propogate_parent(&mut self, parent: ItemId) {
        let mut children = Vec::new();
        let msg = "All items should be defined at this point.";
        match self.all_items[parent.0].0.as_ref().expect(msg) {
            Def0::DBuiltin(builtin) => {
                builtin
                    .get_args()
                    .iter()
                    .for_each(|&arg| children.push(arg));
            }
            Def0::DCompoundType(r#type) => {
                for (_, com) in r#type.get_component_types() {
                    for (_, field) in com.get_fields() {
                        children.push(*field);
                    }
                }
            }
            Def0::DIdentifier(_) => (),
            Def0::DMemberAccess(member) => children.push(member.base()),
            Def0::DParameter(param) => children.push(*param.get_type()),
            Def0::DStructLiteral(r#struct) => {
                for (_, field) in r#struct.fields() {
                    children.push(*field);
                }
            }
            Def0::DUnresolvedSubstitution(sub) => {
                children.push(sub.base());
                for (_, value) in sub.substitutions() {
                    children.push(*value);
                }
            }
        }
        for child in children {
            self.set_parent_and_propogate(child, parent);
        }
    }
}
