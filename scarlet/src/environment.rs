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

pub type Env0 = Environment<Def0>;
pub type Env1 = Environment<Def1>;
pub type Env2 = Environment<Def2>;
pub type Env3 = Environment<Def3>;

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

#[derive(Clone)]
pub struct Environment<Def> {
    language_items: HashMap<String, ItemId>,
    root: ItemId,
    god_type: ItemId,
    all_items: Vec<(Option<Def>, ItemMetadata)>,
    asserts: Vec<Assert>,
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
            write!(f, " ({} deps)", meta.dependencies.len())?;
            if let Some(position) = meta.position {
                write!(f, " ({})", position)?;
            }
            writeln!(f)?;
            if let Some(value) = &meta.value {
                writeln!(f, "{:#?}", value)?;
            } else if let Some(item) = item {
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

impl<Def> IndexMut<ItemId> for Environment<Def> {
    fn index_mut(&mut self, index: ItemId) -> &mut Self::Output {
        if let Some(item) = &mut self.all_items[index.0].0 {
            item
        } else {
            panic!("Item associated with {:?} is undefined.", index)
        }
    }
}

impl<Def: From<DStructLiteral>> Environment<Def> {
    fn new_for_process_result<PreviousDef>(source: &Environment<PreviousDef>) -> Self {
        Self {
            language_items: source.language_items.clone(),
            root: source.root,
            god_type: source.god_type,
            all_items: source
                .all_items
                .iter()
                .map(|(_, meta)| (None, meta.clone()))
                .collect(),
            asserts: source.asserts.clone(),
        }
    }
}

impl<Def> Environment<Def> {
    pub fn get_deps(&self, item: ItemId) -> &HashSet<ParameterPtr> {
        &self.all_items[item.0].1.dependencies
    }

    pub fn type_of(&self, value: ItemId) -> ItemId {
        self.all_items[value.0].1.r#type.unwrap()
    }

    pub fn get_position(&self, item: ItemId) -> Option<Position> {
        self.all_items[item.0].1.position
    }

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

    pub fn is_defined(&self, item: ItemId) -> bool {
        self.all_items[item.0].0.is_some()
    }
}

impl Environment<Def0> {
    pub(crate) fn new() -> Self {
        let root = ItemId(0);
        let god_type = ItemId(1);
        let mut this = Self {
            language_items: HashMap::new(),
            root,
            god_type,
            all_items: vec![(None, ItemMetadata::new()), (None, ItemMetadata::new())],
            asserts: vec![],
        };
        this.define_item(root, DStructLiteral::new_module(vec![]));
        this.define_item(god_type, DBuiltin::god_type());
        this
    }

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

    pub fn processed(&self) -> Env1 {
        let mut target = Environment::new_for_process_result(&self);
        Process0 {
            source: self,
            target: &mut target,
        }
        .process();
        target
    }
}

impl Env1 {
    pub fn dereference(&self, id: ItemId) -> ItemId {
        if let Def1::DOther(DOther(id)) = self[id] {
            self.dereference(id)
        } else {
            id
        }
    }

    pub fn processed(&self) -> Env2 {
        let mut target = Environment::new_for_process_result(&self);
        Process1 {
            source: self,
            target: &mut target,
        }
        .process();
        target
    }
}

impl Env2 {
    pub fn dereference(&self, id: ItemId) -> ItemId {
        if let Def2::DOther(DOther(id)) = self[id] {
            self.dereference(id)
        } else {
            id
        }
    }

    pub fn processed(&self) -> Result<Env3, Vec<Diagnostic>> {
        let mut target = Environment::new_for_process_result(&self);
        let diagnostics = Process2 {
            source: self,
            target: &mut target,
        }
        .process();
        if diagnostics.len() > 0 {
            Err(diagnostics)
        } else {
            Ok(target)
        }
    }
}

impl Def3 {
    fn add_type_asserts(&self, env: &mut Env3) {
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

impl Env3 {
    pub fn dereference(&self, id: ItemId) -> ItemId {
        if let Def3::DOther(DOther(id)) = self[id] {
            self.dereference(id)
        } else {
            id
        }
    }

    pub fn assert_of_type(&mut self, item: ItemId, supertype: ItemId) {
        let original_item = item;
        let item = self.dereference(item);
        let type_of_item = self.all_items[item.0].1.r#type.unwrap();
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
        let assert = self.new_defined_item(def);
        self.asserts.push(Assert {
            condition_which_must_be_true: assert,
            error_when_not: message,
        });
    }
}

struct Process0<'a, 'b> {
    source: &'a Env0,
    target: &'b mut Env1,
}

impl<'a, 'b> Process0<'a, 'b> {
    fn process(&mut self) {
        for index in 0..self.source.all_items.len() {
            let id = ItemId(index);
            self.process_item(id).unwrap();
        }
        self.target.assert_all_defined();
    }

    fn process_item(&mut self, item: ItemId) -> Result<(), ()> {
        if self.target.is_defined(item) {
            return Ok(());
        }
        // TODO: Error on recursion.
        match &self.source[item] {
            Def0::DBuiltin(d) => self.target.define_item(item, d.clone()),
            Def0::DCompoundType(d) => self.target.define_item(item, d.clone()),
            Def0::DIdentifier(ident) => self.process_identifier(item, ident),
            Def0::DUnresolvedMemberAccess(d) => self.target.define_item(item, d.clone()),
            Def0::DParameter(d) => self.target.define_item(item, d.clone()),
            Def0::DStructLiteral(d) => self.target.define_item(item, d.clone()),
            Def0::DUnresolvedSubstitution(d) => self.process_unresolved_substitution(item, d),
        }
        Ok(())
    }

    fn process_identifier(&mut self, this: ItemId, ident: &DIdentifier) {
        let parent = self.source.parent(this).unwrap();
        let target = self.lookup_identifier(parent, ident.identifier());
        self.target.define_item(this, DOther(target.unwrap()));
    }

    fn process_unresolved_substitution(&mut self, this: ItemId, sub: &DUnresolvedSubstitution) {
        let base = sub.base();
        let subs = sub
            .substitutions()
            .iter()
            .map(|(target, value)| {
                let target = match target {
                    UnresolvedTarget::Positional => PartiallyResolvedTarget::Positional,
                    UnresolvedTarget::Named(name) => PartiallyResolvedTarget::Item(
                        self.lookup_identifier(this, name).expect("TODO Nice error"),
                    ),
                };
                (target, *value)
            })
            .collect();
        self.target
            .define_item(this, DPartiallyResolvedSubstitution::new(base, subs));
    }

    fn lookup_identifier(&self, context: ItemId, ident: &str) -> Option<ItemId> {
        if let Def0::DStructLiteral(lit) = &self.source[context] {
            if let Some(field) = lit.get_field(ident) {
                return Some(field);
            }
        }
        if let Some(parent) = self.source.parent(context) {
            self.lookup_identifier(parent, ident)
        } else {
            None
        }
    }
}

struct Process1<'a, 'b> {
    source: &'a Env1,
    target: &'b mut Env2,
}

impl<'a, 'b> Process1<'a, 'b> {
    fn process(&mut self) {
        for index in 0..self.source.all_items.len() {
            let id = ItemId(index);
            self.process_item(id).unwrap();
        }
        self.target.assert_all_defined();
        loop {
            let mut anything_changed = false;
            for index in 0..self.source.all_items.len() {
                let id = ItemId(index);
                anything_changed |= self.compute_deps(id);
            }
            if !anything_changed {
                break;
            }
        }
    }

    fn compute_deps(&mut self, item: ItemId) -> bool {
        let mut deps = HashSet::new();
        match &self.target[item] {
            Def2::DBuiltin(d) => {
                for &arg in d.get_args() {
                    deps.extend(self.target.get_deps(arg).iter().cloned());
                }
            }
            Def2::DCompoundType(d) => {
                for (_, subtype) in d.get_component_types() {
                    if subtype.is_constructable_type() {
                        deps.extend(subtype.parameters(&self.target).into_iter());
                    }
                }
            }
            Def2::DConstructor(con) => {
                let Def2::DCompoundType(r#type) = &self.target[con.r#type()] else { unreachable!() };
                for (_, r#type) in r#type.get_component_types().clone() {
                    for &(_, parameter) in r#type.get_constructor_parameters() {
                        deps.extend(self.target.get_deps(parameter).iter().cloned());
                    }
                }
            }
            Def2::DUnresolvedMemberAccess(d) => {
                deps.extend(self.target.get_deps(d.base()).iter().cloned());
            }
            Def2::DOther(d) => {
                deps.extend(self.target.get_deps(d.0).iter().cloned());
            }
            Def2::DParameter(d) => {
                deps.insert(d.get_parameter_ptr());
            }
            Def2::DStructLiteral(d) => {
                if !d.is_module() {
                    deps.extend(
                        d.fields()
                            .iter()
                            .flat_map(|&(_, field)| self.target.get_deps(field).iter().cloned()),
                    );
                }
            }
            Def2::DPartiallyResolvedSubstitution(d) => {
                let base = self.target.get_deps(d.base()).iter().cloned().collect_vec();
                let mut base = base;
                base.sort_by_key(|p| p.order());
                for (target, value) in d.substitutions() {
                    deps.extend(self.target.get_deps(*value).iter().cloned());
                    match target {
                        PartiallyResolvedTarget::Positional => {
                            if base.len() > 0 {
                                base.remove(0);
                            }
                        }
                        &PartiallyResolvedTarget::Item(target) => {
                            let Def2::DParameter(p) = &self.target[target] else { todo!("Nice error") };
                            let target = p.get_parameter_ptr();
                            if let Some(index) = base.iter().position(|x| x == &target) {
                                base.remove(index);
                            }
                        }
                    }
                }
                deps.extend(base);
            }
        };
        let original = &mut self.target.all_items[item.0].1.dependencies;
        if original.intersection(&deps).count() != deps.len() {
            *original = deps;
            true
        } else {
            false
        }
    }

    fn process_item(&mut self, item: ItemId) -> Result<(), ()> {
        if self.target.is_defined(item) {
            return Ok(());
        }
        match &self.source[item] {
            Def1::DBuiltin(d) => self.target.define_item(item, d.clone()),
            Def1::DCompoundType(d) => self.target.define_item(item, d.clone()),
            Def1::DOther(d) => self.target.define_item(item, d.clone()),
            Def1::DUnresolvedMemberAccess(d) => self.process_member_access(item, d),
            Def1::DParameter(d) => self.target.define_item(item, d.clone()),
            Def1::DStructLiteral(d) => self.target.define_item(item, d.clone()),
            Def1::DPartiallyResolvedSubstitution(d) => self.target.define_item(item, d.clone()),
        }
        Ok(())
    }

    fn process_member_access(&mut self, this: ItemId, access: &DUnresolvedMemberAccess) {
        let base = self.source.dereference(access.base());
        if let Def1::DStructLiteral(module) = &self.source[base] {
            let item = module.get_field(access.member_name()).unwrap();
            self.target.define_item(this, DOther(item));
        } else {
            self.target.define_item(this, access.clone());
        }
    }
}

struct Process2<'a, 'b> {
    source: &'a Env2,
    target: &'b mut Env3,
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
    pub fn into_item(self, env: &mut Env3) -> ItemId {
        let def = self.into_def(env);
        env.new_defined_item(def)
    }

    pub fn into_def(self, env: &mut Env3) -> Def3 {
        match self {
            ConstValue::Type { r#type, arguments } => {
                let base = r#type;
                if arguments.len() == 0 {
                    base.into()
                } else {
                    let base = env.new_defined_item(base);
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
                    let constructor =
                        env.new_defined_item(Def3::DConstructor(DConstructor::new(r#type)));
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

impl<'a, 'b> Process2<'a, 'b> {
    #[must_use]
    fn process(&mut self) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        for index in 0..self.source.all_items.len() {
            let id = ItemId(index);
            if let Err(err) = self.process_item(id) {
                diagnostics.push(err);
            }
        }
        if diagnostics.len() > 0 {
            return diagnostics;
        }
        self.target.assert_all_defined();
        let mut type_index = 0;
        while type_index < self.target.all_items.len() {
            let id = ItemId(type_index);
            self.get_type(id);
            type_index += 1;
        }
        let mut index = 0;
        while index < self.target.all_items.len() {
            let id = ItemId(index);
            let item = self.target[id].clone();
            item.add_type_asserts(&mut self.target);
            index += 1;
            while type_index < self.target.all_items.len() {
                let id = ItemId(type_index);
                self.get_type(id);
                type_index += 1;
            }
        }
        let mut index = 0;
        while index < self.target.all_items.len() {
            let id = ItemId(index);
            self.const_fold(id, HashMap::new());
            index += 1;
        }
        self.target.assert_all_defined();
        let mut errors = Vec::new();
        for assert in &self.target.asserts {
            let condition = &self.target.all_items[assert.condition_which_must_be_true.0];
            if let &Some(ConstValue::Value { r#type, .. }) = &condition.1.value {
                if r#type == self.target.get_language_item("False").unwrap() {
                    errors.push(self.make_error(assert));
                }
            } else {
                errors.push(self.make_error(assert));
            }
        }
        errors
    }

    fn make_error(&self, assert: &Assert) -> Diagnostic {
        match &assert.error_when_not {
            &AssertMessage::ItemTypeMustBeSubtype {
                type_of,
                must_be_subtype_of,
            } => Diagnostic::new()
                .with_text_error("The following expression:".to_owned())
                .with_item_error(type_of, &self.target)
                .with_text_error("Must be of the following type:".to_owned())
                .with_item_error(must_be_subtype_of, &self.target),
        }
    }

    fn get_type(&mut self, item: ItemId) -> ItemId {
        if let Some(r#type) = &self.target.all_items[item.0].1.r#type {
            *r#type
        } else {
            let r#type = self.type_of(item);
            self.const_fold(r#type, HashMap::new());
            self.target.all_items[item.0].1.r#type = Some(r#type);
            r#type
        }
    }

    fn type_of(&mut self, item: ItemId) -> ItemId {
        match &self.target[item] {
            Def3::DBuiltin(d) => match d.get_builtin() {
                Builtin::IsExactly | Builtin::IsSubtypeOf => {
                    self.target.get_language_item("Bool").unwrap()
                }
                Builtin::IfThenElse => d.get_args()[0],
                Builtin::Union | Builtin::GodType => self.target.god_type(),
            },
            Def3::DCompoundType(_) => self.target.god_type(),
            Def3::DConstructor(d) => d.r#type(),
            Def3::DUnresolvedMemberAccess(d) => {
                let d = d.clone();
                let base_type = self.get_type(d.base());
                let Some(ConstValue::Type { r#type, arguments }) = self.const_fold(base_type, HashMap::new()) else { panic!() };
                let Some(r#type) = r#type.get_single_type() else { panic!() };
                let fields = r#type.get_constructor_parameters();
                let field = fields
                    .iter()
                    .find(|(name, _)| name == &d.member_name())
                    .unwrap();
                let base = self.get_type(field.1);
                let base_deps = self.target.get_deps(base);
                let filtered_arguments: Vec<_> = arguments
                    .into_iter()
                    .filter(|(param, _)| base_deps.contains(param))
                    .collect();
                if filtered_arguments.len() == 0 {
                    base
                } else {
                    let mut realized_arguments = OrderedMap::new();
                    for (param, arg) in filtered_arguments.into_iter() {
                        realized_arguments
                            .insert(param.ptr_clone(), arg.into_item(&mut self.target));
                    }
                    self.target
                        .new_defined_item(Def3::DSubstitution(DSubstitution::new(
                            base,
                            realized_arguments,
                        )))
                }
            }
            Def3::DOther(d) => self.get_type(d.0),
            Def3::DParameter(d) => d.get_type(),
            Def3::DStructLiteral(d) => {
                if d.is_module() {
                    let d = d.clone();
                    let mut declarations = Vec::new();
                    for field in d.fields() {
                        declarations.push(field.0.clone());
                    }
                    let r#type = Type::ModuleType {
                        type_id: TypeId::UserType(Rc::new(())),
                        declarations,
                    };
                    let r#type = DCompoundType::new_single(Rc::new(r#type));
                    self.target.new_defined_item(r#type)
                } else {
                    todo!()
                }
            }
            Def3::DSubstitution(d) => {
                let subs = d.substitutions().clone();
                let base = self.get_type(d.base());
                let deps = self.target.get_deps(base);
                let subs: OrderedMap<_, _> = subs
                    .into_iter()
                    .filter(|sub| deps.contains(&sub.0))
                    .collect();
                if subs.len() > 0 {
                    let sub = DSubstitution::new(base, subs);
                    self.target.new_defined_item(sub)
                } else {
                    base
                }
            }
        }
    }

    fn const_fold(
        &mut self,
        item: ItemId,
        args: HashMap<ParameterPtr, ConstValue>,
    ) -> Option<ConstValue> {
        let no_args = args.is_empty();
        let value = self.const_fold_inner(item, args);
        if no_args {
            if let Some(value) = &value {
                self.target.all_items[item.0].1.value = Some(value.clone());
            }
        }
        value
    }

    fn const_fold_inner(
        &mut self,
        item: ItemId,
        args: HashMap<ParameterPtr, ConstValue>,
    ) -> Option<ConstValue> {
        match &self.target[item] {
            Def3::DBuiltin(d) => match d.get_builtin() {
                Builtin::IsExactly => {
                    let a = self.target.dereference(d.get_args()[2]);
                    let b = self.target.dereference(d.get_args()[3]);
                    if let (Some(a), Some(b)) = (
                        self.const_fold(a, args.clone()),
                        self.const_fold(b, args.clone()),
                    ) {
                        let return_type = if a == b {
                            self.target.get_language_item("True").unwrap()
                        } else {
                            self.target.get_language_item("False").unwrap()
                        };
                        Some(ConstValue::Value {
                            r#type: return_type,
                            subs: hashmap![],
                        })
                    } else {
                        None
                    }
                }
                Builtin::IsSubtypeOf => {
                    let a = self.target.dereference(d.get_args()[0]);
                    let b = self.target.dereference(d.get_args()[1]);
                    if let (
                        Some(ConstValue::Type {
                            r#type: a,
                            arguments: a_args,
                        }),
                        Some(ConstValue::Type {
                            r#type: b,
                            arguments: b_args,
                        }),
                    ) = (
                        self.const_fold(a, args.clone()),
                        self.const_fold(b, args.clone()),
                    ) {
                        let component_types_check_out = a
                            .get_component_types()
                            .keys()
                            .all(|required_id| b.get_component_types().contains_key(required_id));
                        // If a really is a subtype of b, it will not have any additional parameters
                        // not in b.
                        let arguments_check_out = a_args
                            .iter()
                            .all(|(param, value)| b_args.get(param) == Some(value));
                        let return_type = if component_types_check_out && arguments_check_out {
                            self.target.get_language_item("True").unwrap()
                        } else {
                            self.target.get_language_item("False").unwrap()
                        };
                        Some(ConstValue::Value {
                            r#type: return_type,
                            subs: hashmap![],
                        })
                    } else {
                        None
                    }
                }
                Builtin::IfThenElse => {
                    let true_type = self.target.get_language_item("True").unwrap();
                    let false_type = self.target.get_language_item("False").unwrap();
                    let true_result = d.get_args()[2];
                    let false_result = d.get_args()[3];
                    let condition = self.const_fold(d.get_args()[1], args.clone());
                    if condition
                        == Some(ConstValue::Value {
                            r#type: true_type,
                            subs: hashmap![],
                        })
                    {
                        self.const_fold(true_result, args)
                    } else if condition
                        == Some(ConstValue::Value {
                            r#type: false_type,
                            subs: hashmap![],
                        })
                    {
                        self.const_fold(false_result, args)
                    } else {
                        None
                    }
                }
                Builtin::Union => {
                    let a = self.target.dereference(d.get_args()[0]);
                    let b = self.target.dereference(d.get_args()[1]);
                    if let (
                        Some(ConstValue::Type {
                            r#type: a,
                            arguments: a_args,
                        }),
                        Some(ConstValue::Type {
                            r#type: b,
                            arguments: b_args,
                        }),
                    ) = (self.const_fold(a, args.clone()), self.const_fold(b, args))
                    {
                        let mut args = a_args;
                        args.extend(b_args.into_iter());
                        Some(ConstValue::Type {
                            r#type: a.union(&b),
                            arguments: args,
                        })
                    } else {
                        None
                    }
                }
                Builtin::GodType => Some(ConstValue::Type {
                    r#type: DCompoundType::god_type(),
                    arguments: hashmap![],
                }),
            },
            Def3::DCompoundType(d) => {
                let params = d.parameters(&self.source);
                let mut const_args = HashMap::new();
                for param in params {
                    if let Some(value) = args.get(&param) {
                        const_args.insert(param, value.clone());
                    } else {
                        return None;
                    }
                }
                Some(ConstValue::Type {
                    r#type: d.clone(),
                    arguments: const_args,
                })
            }
            Def3::DConstructor(d) => {
                let deps = &self.target.all_items[item.0].1.dependencies;
                let subs: HashMap<_, _> = args
                    .into_iter()
                    .filter(|x| deps.contains(&x.0))
                    .map(|(a, b)| (a.clone(), b.clone()))
                    .collect();
                if subs.len() < deps.len() {
                    None
                } else {
                    Some(ConstValue::Value {
                        r#type: d.r#type(),
                        subs,
                    })
                }
            }
            Def3::DUnresolvedMemberAccess(d) => {
                let member_name = d.member_name().to_owned();
                if let Some(ConstValue::Value {
                    r#type,
                    subs: values,
                }) = self.const_fold(d.base(), args.clone())
                {
                    let Def3::DCompoundType(r#type) = &self.target[r#type] else { unreachable!() };
                    assert_eq!(r#type.get_component_types().len(), 1);
                    let (_, r#type) = r#type.get_component_types().iter().next().unwrap();
                    let field = r#type
                        .get_constructor_parameters()
                        .iter()
                        .find(|x| x.0 == member_name)
                        .unwrap();
                    self.const_fold(field.1, values.clone())
                } else {
                    None
                }
            }
            Def3::DOther(d) => self.const_fold(d.0, args),
            Def3::DParameter(d) => {
                if let Some(value) = args.get(d.get_parameter()) {
                    Some(value.clone())
                } else {
                    None
                }
            }
            Def3::DStructLiteral(d) => {
                if d.is_module() {
                    None
                } else {
                    todo!()
                }
            }
            Def3::DSubstitution(d) => {
                let d = d.clone();
                let mut new_args = args.clone();
                for (param, arg) in d.substitutions() {
                    let arg = self.const_fold(*arg, args.clone())?;
                    new_args.insert(param.ptr_clone(), arg);
                }
                self.const_fold(d.base(), new_args)
            }
        }
    }

    fn process_item(&mut self, item: ItemId) -> Result<(), Diagnostic> {
        if self.target.is_defined(item) {
            return Ok(());
        }
        match &self.source[item] {
            Def2::DBuiltin(d) => self.target.define_item(item, d.clone()),
            Def2::DCompoundType(d) => self.target.define_item(item, d.clone()),
            Def2::DConstructor(d) => self.target.define_item(item, d.clone()),
            Def2::DOther(d) => self.target.define_item(item, d.clone()),
            Def2::DUnresolvedMemberAccess(d) => self.target.define_item(item, d.clone()),
            Def2::DParameter(d) => self.target.define_item(item, d.clone()),
            Def2::DStructLiteral(d) => self.target.define_item(item, d.clone()),
            Def2::DPartiallyResolvedSubstitution(d) => {
                return self.process_partially_resolved_substitution(item, d)
            }
        }
        Ok(())
    }

    fn process_partially_resolved_substitution(
        &mut self,
        this: ItemId,
        sub: &DPartiallyResolvedSubstitution,
    ) -> Result<(), Diagnostic> {
        let base = sub.base();
        let mut substitutions = Substitutions::new();
        let mut deps = self.source.get_deps(sub.base()).clone();
        for (target, value) in sub.substitutions() {
            let target = match target {
                PartiallyResolvedTarget::Positional => {
                    let min_dep = deps.iter().min_by_key(|dep| dep.order());
                    if let Some(dep) = min_dep {
                        dep.ptr_clone()
                    } else {
                        return Err(Diagnostic::new()
                            .with_text_error(
                                "A substitution requires fewer arguments. All parameters have \
                                 been substituted, leaving this argument with no corresponding \
                                 parameter:"
                                    .to_owned(),
                            )
                            .with_item_error(*value, &self.source));
                    }
                }
                &PartiallyResolvedTarget::Item(target) => {
                    let Def2::DParameter(p) = &self.source[target] else { todo!("Nice error") };
                    p.get_parameter_ptr()
                }
            };
            deps.remove(&target);
            substitutions.insert(target, *value);
        }
        self.target
            .define_item(this, DSubstitution::new(base, substitutions));
        Ok(())
    }
}
