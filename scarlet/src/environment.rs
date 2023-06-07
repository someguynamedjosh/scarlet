use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::{Index, IndexMut},
    rc::Rc,
};

use itertools::Itertools;
use maplit::hashmap;

use crate::{
    definitions::{
        builtin::{Builtin, DBuiltin},
        compound_type::{DCompoundType, Type, TypeId},
        constructor::DConstructor,
        identifier::DIdentifier,
        member_access::{DMemberAccess, DUnresolvedMemberAccess},
        other::DOther,
        parameter::{DParameter, ParameterPtr},
        struct_literal::DStructLiteral,
        substitution::{
            DPartiallyResolvedSubstitution, DSubstitution, DUnresolvedSubstitution,
            PartiallyResolvedTarget, Substitutions, UnresolvedTarget,
        },
    },
    diagnostic::{Diagnostic, Position},
    item::query::{Query, QueryContext, RootQuery},
    shared::OrderedMap,
    util::PtrExtension,
};

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
    DUnresolvedMemberAccess,
    DParameter,
    DStructLiteral,
    DUnresolvedSubstitution
});

def_enum!(Def1 {
    DBuiltin,
    DCompoundType,
    DUnresolvedMemberAccess,
    DOther,
    DParameter,
    DStructLiteral,
    DPartiallyResolvedSubstitution
});

def_enum!(Def2 {
    DBuiltin,
    DCompoundType,
    DConstructor,
    DUnresolvedMemberAccess,
    DOther,
    DParameter,
    DStructLiteral,
    DPartiallyResolvedSubstitution
});

def_enum!(Def3 {
    DBuiltin,
    DCompoundType,
    DConstructor,
    DUnresolvedMemberAccess,
    DOther,
    DParameter,
    DStructLiteral,
    DSubstitution
});

def_enum!(Def4 {
    DBuiltin,
    DCompoundType,
    DConstructor,
    DMemberAccess,
    DOther,
    DParameter,
    DStructLiteral,
    DSubstitution
});

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
    pub dependencies: HashSet<ParameterPtr>,
    pub r#type: Option<ItemId>,
    pub value: Option<ConstValue>,
}

impl ItemMetadata {
    pub fn new() -> Self {
        Self {
            parent: None,
            position: None,
            dependencies: HashSet::new(),
            r#type: None,
            value: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssertMessage {
    ItemTypeMustBeSubtype {
        type_of: ItemId,
        must_be_subtype_of: ItemId,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assert {
    condition_which_must_be_true: ItemId,
    error_when_not: AssertMessage,
}

#[derive(Clone, Default, Debug)]
pub struct Item {
    def0: Option<Def0>,
    def1: Option<Def1>,
    def2: Option<Def2>,
    def3: Option<Def3>,
    def4: Option<Def4>,
    parent: Option<ItemId>,
    position: Option<Position>,
    dependencies: Option<HashSet<ParameterPtr>>,
    r#type: Option<ItemId>,
    value: Option<Value>,
}

#[derive(Clone)]
pub struct Env {
    language_items: HashMap<String, ItemId>,
    root: ItemId,
    god_type: ItemId,
    all_items: Vec<Item>,
    asserts: Vec<Assert>,
}

impl Debug for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Environment (Root {:?})", self.root)?;
        writeln!(f)?;
        writeln!(f, "Language Items:")?;
        for (key, value) in &self.language_items {
            writeln!(f, "{:?} => {:#?}", key, value)?;
        }
        writeln!(f)?;
        writeln!(f, "Items:")?;
        for (key, item) in self.all_items.iter().enumerate() {
            write!(f, "I#{}", key)?;
            if let Some(parent) = item.parent {
                write!(f, " (Child of {:?})", parent)?;
            }
            if let Some(deps) = &item.dependencies {
                write!(f, " ({} deps)", deps.len())?;
            }
            if let Some(position) = item.position {
                write!(f, " ({})", position)?;
            }
            writeln!(f)?;
            if let Some(value) = &item.value {
                writeln!(f, "{:#?}", value)?;
            } else if let Some(item) = &item.def4 {
                writeln!(f, "{:#?}", item)?;
            } else if let Some(item) = &item.def3 {
                writeln!(f, "{:#?}", item)?;
            } else if let Some(item) = &item.def2 {
                writeln!(f, "{:#?}", item)?;
            } else if let Some(item) = &item.def1 {
                writeln!(f, "{:#?}", item)?;
            } else if let Some(item) = &item.def0 {
                writeln!(f, "{:#?}", item)?;
            } else {
                writeln!(f, "Undefined")?;
            }
        }
        Ok(())
    }
}

impl Index<ItemId> for Env {
    type Output = Item;

    fn index(&self, index: ItemId) -> &Self::Output {
        &self.all_items[index.0]
    }
}

impl IndexMut<ItemId> for Env {
    fn index_mut(&mut self, index: ItemId) -> &mut Self::Output {
        &mut self.all_items[index.0]
    }
}

impl Env {
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

    pub fn get_language_item(&self, name: &str) -> Result<ItemId, Diagnostic> {
        self.language_items.get(name).copied().ok_or_else(|| {
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

    pub fn god_type(&self) -> ItemId {
        self.god_type
    }

    pub(crate) fn new() -> Self {
        let root = ItemId(0);
        let god_type = ItemId(1);
        let mut this = Self {
            language_items: HashMap::new(),
            root,
            god_type,
            all_items: vec![],
            asserts: vec![],
        };
        let actual_root = this.define0(DStructLiteral::new_module(vec![]));
        let actual_god_type = this.define0(DBuiltin::god_type());
        assert_eq!(root, actual_root);
        assert_eq!(god_type, actual_god_type);
        this
    }

    pub fn compute_parents(&mut self) {
        self.propogate_parent(self.root)
    }

    fn set_parent_and_propogate(&mut self, child: ItemId, parent: ItemId) {
        self[child].parent = Some(parent);
        self.propogate_parent(child);
    }

    fn propogate_parent(&mut self, parent: ItemId) {
        let mut children = Vec::new();
        let msg = "All items should be defined at this point.";
        match self[parent].def0.as_ref().expect(msg) {
            Def0::DBuiltin(builtin) => {
                builtin
                    .get_args()
                    .iter()
                    .for_each(|&arg| children.push(arg));
            }
            Def0::DCompoundType(r#type) => {
                for (_, com) in r#type.get_component_types() {
                    if com.is_constructable_type() {
                        for (_, field) in com.get_constructor_parameters() {
                            children.push(*field);
                        }
                    }
                }
            }
            Def0::DIdentifier(_) => (),
            Def0::DUnresolvedMemberAccess(member) => children.push(member.base()),
            Def0::DParameter(param) => children.push(param.get_type()),
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

    pub fn dereference(&self, id: ItemId) -> ItemId {
        let item = &self[id];
        if let &Some(Def3::DOther(DOther(id))) = &item.def3 {
            self.dereference(id)
        } else if let &Some(Def2::DOther(DOther(id))) = &item.def2 {
            self.dereference(id)
        } else if let &Some(Def1::DOther(DOther(id))) = &item.def1 {
            self.dereference(id)
        } else {
            id
        }
    }

    pub fn assert_of_type(&mut self, item: ItemId, supertype: ItemId) {
        let original_item = item;
        let item = self.dereference(item);
        let type_of_item = self.all_items[item.0].r#type.unwrap();
        self.assert_subtype(
            type_of_item,
            supertype,
            AssertMessage::ItemTypeMustBeSubtype {
                type_of: original_item,
                must_be_subtype_of: supertype,
            },
        );
    }

    pub fn assert_subtype(&mut self, subtype: ItemId, supertype: ItemId, message: AssertMessage) {
        let subtype = self.dereference(subtype);
        let supertype = self.dereference(supertype);
        if subtype == supertype {
            // This avoids an infinite loop of asserting that type is a type.
            return;
        }
        let def = DBuiltin::is_subtype_of(subtype, supertype);
        let assert = self.define0(def);
        self.asserts.push(Assert {
            condition_which_must_be_true: assert,
            error_when_not: message,
        });
    }

    pub fn define0(&mut self, definition: impl Into<Def0>) -> ItemId {
        let id = ItemId(self.all_items.len());
        self.all_items.push(Item {
            def0: Some(definition.into()),
            ..Item::default()
        });
        id
    }

    pub fn define3(&mut self, definition: impl Into<Def3>) -> ItemId {
        let id = ItemId(self.all_items.len());
        self.all_items.push(Item {
            def3: Some(definition.into()),
            ..Item::default()
        });
        id
    }
}

impl Env {
    pub fn get_def0(&self, item: ItemId) -> &Def0 {
        self[item].def0.as_ref().unwrap()
    }

    pub fn get_def1(&mut self, item: ItemId) -> &Def1 {
        todo!()
    }

    pub fn get_def2(&mut self, item: ItemId) -> &Def2 {
        todo!()
    }

    pub fn get_def3(&mut self, item: ItemId) -> &Def3 {
        todo!()
    }

    pub fn get_def4(&mut self, item: ItemId) -> &Def4 {
        todo!()
    }

    pub fn get_parent(&self, item: ItemId) -> Option<ItemId> {
        self[item].parent
    }

    pub fn get_position(&self, item: ItemId) -> Option<Position> {
        self[item].position
    }

    pub fn set_position(&mut self, item: ItemId, position: Position) {
        self[item].position = Some(position);
    }

    pub fn get_deps(&mut self, item: ItemId) -> &HashSet<ParameterPtr> {
        todo!()
    }

    pub fn get_type(&mut self, item: ItemId) -> ItemId {
        todo!()
    }

    pub fn get_value(&mut self, item: ItemId) -> &Value {
        todo!()
    }
}

impl Def3 {
    fn add_type_asserts(&self, env: &mut Env) {
        match self {
            Def3::DBuiltin(d) => d.add_type_asserts(env),
            Def3::DCompoundType(..) => {}
            Def3::DConstructor(d) => d.add_type_asserts(env),
            Def3::DUnresolvedMemberAccess(_) => {}
            Def3::DParameter(d) => d.add_type_asserts(env),
            Def3::DSubstitution(d) => d.add_type_asserts(env),
            Def3::DStructLiteral(..) => {}
            Def3::DOther(_) => (),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Known(ConstValue),
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstValue {
    Type {
        r#type: DCompoundType,
        arguments: HashMap<ParameterPtr, ConstValue>,
    },
    Value {
        r#type: ItemId,
        subs: HashMap<ParameterPtr, ConstValue>,
    },
}

impl ConstValue {
    pub fn into_item(self, env: &mut Env) -> ItemId {
        let def = self.into_def(env);
        env.define3(def)
    }

    pub fn into_def(self, env: &mut Env) -> Def3 {
        match self {
            ConstValue::Type { r#type, arguments } => {
                let base = r#type;
                if arguments.len() == 0 {
                    base.into()
                } else {
                    let base = env.define0(base);
                    Def3::DSubstitution(DSubstitution::new(
                        base,
                        arguments
                            .into_iter()
                            .map(|(param, arg)| (param.ptr_clone(), arg.into_item(env)))
                            .collect(),
                    ))
                }
            }
            ConstValue::Value { r#type, subs } => {
                if subs.len() == 0 {
                    Def3::DConstructor(DConstructor::new(r#type))
                } else {
                    let constructor = env.define3(DConstructor::new(r#type));
                    Def3::DSubstitution(DSubstitution::new(
                        constructor,
                        subs.into_iter()
                            .map(|(param, arg)| (param.ptr_clone(), arg.into_item(env)))
                            .collect(),
                    ))
                }
            }
        }
    }
}
