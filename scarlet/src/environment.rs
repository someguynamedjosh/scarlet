use std::{
    borrow::Cow,
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
        invalid::DInvalid,
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
    ($Name:ident { $($Variant:ident,)* }) => {
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
    DUnresolvedSubstitution,
});

def_enum!(Def1 {
    DBuiltin,
    DCompoundType,
    DUnresolvedMemberAccess,
    DOther,
    DParameter,
    DStructLiteral,
    DPartiallyResolvedSubstitution,
    DInvalid,
});

def_enum!(Def2 {
    DBuiltin,
    DCompoundType,
    DUnresolvedMemberAccess,
    DOther,
    DParameter,
    DStructLiteral,
    DPartiallyResolvedSubstitution,
    DInvalid,
});

def_enum!(Def3 {
    DBuiltin,
    DCompoundType,
    DConstructor,
    DMemberAccess,
    DOther,
    DParameter,
    DStructLiteral,
    DSubstitution,
    DInvalid,
});

def_enum!(Def4 {
    DBuiltin,
    DCompoundType,
    DConstructor,
    DMemberAccess,
    DOther,
    DParameter,
    DStructLiteral,
    DSubstitution,
    DInvalid,
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
    diagnostics: Vec<Diagnostic>,
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
            if let Some(Value::Known(value)) = &item.value {
                writeln!(f, "Value::Known({:#?})", value)?;
            } else if let Some(item) = &item.def4 {
                writeln!(f, "Def4::{:#?}", item)?;
            } else if let Some(item) = &item.def3 {
                writeln!(f, "Def3::{:#?}", item)?;
            } else if let Some(item) = &item.def2 {
                writeln!(f, "Def2::{:#?}", item)?;
            } else if let Some(item) = &item.def1 {
                writeln!(f, "Def1::{:#?}", item)?;
            } else if let Some(item) = &item.def0 {
                writeln!(f, "Def0::{:#?}", item)?;
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
            diagnostics: vec![],
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

    fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
}

impl Env {
    pub fn get_def0(&self, item: ItemId) -> &Def0 {
        self[item].def0.as_ref().unwrap()
    }

    pub fn lazy_get_def1(&self, item: ItemId) -> &Option<Def1> {
        &self[item].def1
    }

    pub fn lazy_get_def2(&self, item: ItemId) -> &Option<Def2> {
        &self[item].def2
    }

    pub fn lazy_get_def3(&self, item: ItemId) -> &Option<Def3> {
        &self[item].def3
    }

    pub fn lazy_get_def4(&self, item: ItemId) -> &Option<Def4> {
        &self[item].def4
    }

    pub fn get_def1(&mut self, item: ItemId) -> &Def1 {
        if self[item].def1.is_none() {
            self[item].def1 = Some(self.compute_def1(item));
        }
        self[item].def1.as_ref().unwrap()
    }

    pub fn get_def2(&mut self, item: ItemId) -> &Def2 {
        if self[item].def2.is_none() {
            self[item].def2 = Some(self.compute_def2(item));
        }
        self[item].def2.as_ref().unwrap()
    }

    pub fn get_def3(&mut self, item: ItemId) -> &Def3 {
        if self[item].def3.is_none() {
            self[item].def3 = Some(self.compute_def3(item));
        }
        self[item].def3.as_ref().unwrap()
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
        if self[item].dependencies.is_none() {
            self[item].dependencies = Some(self.compute_deps(item));
        }
        self[item].dependencies.as_ref().unwrap()
    }

    pub fn get_type(&mut self, item: ItemId) -> ItemId {
        if self[item].r#type.is_none() {
            self[item].r#type = Some(self.compute_type(item));
        }
        self[item].r#type.unwrap()
    }

    pub fn get_value(&mut self, item: ItemId) -> &Value {
        if self[item].value.is_none() {
            self[item].value = Some(self.compute_value(item, HashMap::new()));
        }
        self[item].value.as_ref().unwrap()
    }

    pub fn get_value_with_args(
        &mut self,
        item: ItemId,
        args: HashMap<ParameterPtr, ConstValue>,
    ) -> Cow<Value> {
        if args.len() == 0 {
            Cow::Borrowed(self.get_value(item))
        } else {
            Cow::Owned(self.compute_value(item, args))
        }
    }

    pub fn get_member(&self, module: ItemId, member: &str) -> Option<ItemId> {
        let module = self.dereference(module);
        if let Def0::DStructLiteral(r#struct) = self.get_def0(module) {
            r#struct
                .fields()
                .iter()
                .find(|(name, _)| name == member)
                .map(|(_, item)| *item)
        } else {
            None
        }
    }
}

impl Env {
    fn compute_def1(&mut self, item: ItemId) -> Def1 {
        match self.get_def0(item) {
            Def0::DBuiltin(d) => d.clone().into(),
            Def0::DCompoundType(d) => d.clone().into(),
            Def0::DIdentifier(d) => self.process_identifier(item, &d.clone()),
            Def0::DUnresolvedMemberAccess(d) => d.clone().into(),
            Def0::DParameter(d) => d.clone().into(),
            Def0::DStructLiteral(d) => d.clone().into(),
            Def0::DUnresolvedSubstitution(d) => {
                self.process_unresolved_substitution(item, &d.clone())
            }
        }
    }

    fn process_identifier(&mut self, this: ItemId, ident: &DIdentifier) -> Def1 {
        let parent = self.get_parent(this).unwrap();
        let target = self.lookup_identifier(parent, ident.identifier());
        DOther(target.unwrap()).into()
    }

    fn process_unresolved_substitution(
        &mut self,
        this: ItemId,
        sub: &DUnresolvedSubstitution,
    ) -> Def1 {
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
        DPartiallyResolvedSubstitution::new(base, subs).into()
    }

    fn lookup_identifier(&self, context: ItemId, ident: &str) -> Option<ItemId> {
        if let Def0::DStructLiteral(lit) = &self.get_def0(context) {
            if let Some(field) = lit.get_field(ident) {
                return Some(field);
            }
        }
        if let Some(parent) = self.get_parent(context) {
            self.lookup_identifier(parent, ident)
        } else {
            None
        }
    }
}

impl Env {
    fn compute_def2(&mut self, item: ItemId) -> Def2 {
        self.get_def1(item);
        match self.lazy_get_def1(item).as_ref().unwrap() {
            Def1::DBuiltin(d) => d.clone().into(),
            Def1::DCompoundType(d) => d.clone().into(),
            Def1::DUnresolvedMemberAccess(d) => d.clone().into(),
            Def1::DOther(d) => d.clone().into(),
            Def1::DParameter(d) => d.clone().into(),
            Def1::DStructLiteral(d) => d.clone().into(),
            Def1::DPartiallyResolvedSubstitution(d) => d.clone().into(),
            Def1::DInvalid(d) => d.clone().into(),
        }
    }
}

impl Env {
    fn compute_def3(&mut self, item: ItemId) -> Def3 {
        self.get_def2(item);
        match self.lazy_get_def2(item).as_ref().unwrap() {
            Def2::DBuiltin(d) => d.clone().into(),
            Def2::DCompoundType(d) => d.clone().into(),
            Def2::DUnresolvedMemberAccess(d) => self.process_unresolved_member_access(&d.clone()),
            Def2::DOther(d) => d.clone().into(),
            Def2::DParameter(d) => d.clone().into(),
            Def2::DStructLiteral(d) => d.clone().into(),
            Def2::DPartiallyResolvedSubstitution(sub) => {
                self.process_partially_resolved_substitution(&sub.clone())
            }
            Def2::DInvalid(d) => d.clone().into(),
        }
    }

    fn process_unresolved_member_access(&mut self, member: &DUnresolvedMemberAccess) -> Def3 {
        let base = member.base();
        let member_name = member.member_name();
        let base_type_item = self.get_type(base);
        let Value::Known(ConstValue::Type { r#type, arguments }) = self.get_value(base_type_item) else { todo!() };
        let Some(r#type) = r#type.get_single_type() else { todo!() };
        if r#type.is_god_type() {
            if member_name == "new" {
                DConstructor::new(base).into()
            } else {
                todo!()
            }
        } else {
            DMemberAccess::new(base, r#type, member_name)
                .unwrap()
                .into()
        }
    }

    fn process_partially_resolved_substitution(
        &mut self,
        sub: &DPartiallyResolvedSubstitution,
    ) -> Def3 {
        let base = sub.base();
        let mut substitutions = Substitutions::new();
        let mut deps = self.get_deps(sub.base()).clone();
        for (target, value) in sub.substitutions() {
            let target = match target {
                PartiallyResolvedTarget::Positional => {
                    let min_dep = deps.iter().min_by_key(|dep| dep.order());
                    if let Some(dep) = min_dep {
                        dep.ptr_clone()
                    } else {
                        self.push_diagnostic(
                            Diagnostic::new()
                                .with_text_error(
                                    "A substitution requires fewer arguments. All parameters have \
                                     been substituted, leaving this argument with no \
                                     corresponding parameter:"
                                        .to_owned(),
                                )
                                .with_item_error(*value, self),
                        );
                        return DInvalid.into();
                    }
                }
                &PartiallyResolvedTarget::Item(target) => {
                    let Def2::DParameter(p) = self.get_def2(target) else { todo!("Nice error") };
                    p.get_parameter_ptr()
                }
            };
            deps.remove(&target);
            substitutions.insert(target, *value);
        }
        DSubstitution::new(base, substitutions).into()
    }
}

impl Env {
    fn compute_deps(&mut self, item: ItemId) -> HashSet<ParameterPtr> {
        let mut deps = HashSet::new();
        self.get_def3(item);
        match self.lazy_get_def3(item).as_ref().unwrap() {
            Def3::DBuiltin(d) => {
                for &arg in d.clone().get_args() {
                    deps.extend(self.get_deps(arg).iter().cloned());
                }
            }
            Def3::DCompoundType(d) => {
                for (_, subtype) in d.clone().get_component_types() {
                    if subtype.is_constructable_type() {
                        deps.extend(subtype.parameters(self).into_iter());
                    }
                }
            }
            Def3::DConstructor(con) => {
                let Value::Known(ConstValue::Type { r#type, .. }) = &self.get_value(con.r#type()) else { unreachable!() };
                let Some(r#type) = r#type.get_single_type() else { unreachable!() };
                for &(_, parameter) in r#type.ptr_clone().get_constructor_parameters() {
                    deps.extend(self.get_deps(parameter).iter().cloned());
                }
            }
            Def3::DMemberAccess(d) => {
                deps.extend(self.get_deps(d.base()).iter().cloned());
            }
            Def3::DOther(d) => {
                deps.extend(self.get_deps(d.0).iter().cloned());
            }
            Def3::DParameter(d) => {
                deps.insert(d.get_parameter_ptr());
            }
            Def3::DStructLiteral(d) => {
                if !d.is_module() {
                    let d = d.clone();
                    for &(_, field) in d.fields().iter() {
                        deps.extend(self.get_deps(field).iter().cloned());
                    }
                }
            }
            Def3::DSubstitution(d) => {
                let d = d.clone();
                let base = self.get_deps(d.base()).iter().cloned().collect_vec();
                let mut base = base;
                base.sort_by_key(|p| p.order());
                for (target, value) in d.substitutions() {
                    deps.extend(self.get_deps(*value).iter().cloned());
                    if let Some(index) = base.iter().position(|x| x == target) {
                        base.remove(index);
                    }
                }
                deps.extend(base);
            }
            Def3::DInvalid(..) => (),
        };
        deps
    }
}

impl Env {
    fn compute_type(&mut self, item: ItemId) -> ItemId {
        self.get_def3(item);
        match self.lazy_get_def3(item).as_ref().unwrap() {
            Def3::DBuiltin(d) => match d.get_builtin() {
                Builtin::IsExactly | Builtin::IsSubtypeOf => {
                    self.get_language_item("Bool").unwrap()
                }
                Builtin::IfThenElse => d.get_args()[0],
                Builtin::Union | Builtin::GodType => self.god_type(),
            },
            Def3::DCompoundType(_) => self.god_type(),
            Def3::DConstructor(d) => d.r#type(),
            Def3::DMemberAccess(d) => {
                let d = d.clone();
                let base_type = self.get_type(d.base());
                let Value::Known(ConstValue::Type { r#type, arguments }) = self.get_value(base_type).clone() else { panic!() };
                let Some(r#type) = r#type.get_single_type() else { panic!() };
                let fields = r#type.get_constructor_parameters();
                let field = &fields[d.member_index()];
                let base = self.get_type(field.1);
                let base_deps = self.get_deps(base);
                let filtered_arguments: Vec<_> = arguments
                    .into_iter()
                    .filter(|(param, _)| base_deps.contains(param))
                    .collect();
                if filtered_arguments.len() == 0 {
                    base
                } else {
                    let mut realized_arguments = OrderedMap::new();
                    for (param, arg) in filtered_arguments.into_iter() {
                        realized_arguments.insert(param.ptr_clone(), arg.into_item(self));
                    }
                    self.define3(Def3::DSubstitution(DSubstitution::new(
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
                    self.define3(r#type)
                } else {
                    todo!()
                }
            }
            Def3::DSubstitution(d) => {
                let subs = d.substitutions().clone();
                let base = self.get_type(d.base());
                let deps = self.get_deps(base);
                let subs: OrderedMap<_, _> = subs
                    .into_iter()
                    .filter(|sub| deps.contains(&sub.0))
                    .collect();
                if subs.len() > 0 {
                    let sub = DSubstitution::new(base, subs);
                    self.define3(sub)
                } else {
                    base
                }
            }
            Def3::DInvalid(..) => item,
        }
    }
}

impl Env {
    fn compute_value(&mut self, item: ItemId, args: HashMap<ParameterPtr, ConstValue>) -> Value {
        use Value::*;
        self.get_def3(item);
        match self.lazy_get_def3(item).as_ref().unwrap() {
            Def3::DBuiltin(d) => match d.get_builtin() {
                Builtin::IsExactly => {
                    let a = self.dereference(d.get_args()[2]);
                    let b = self.dereference(d.get_args()[3]);
                    if let (Known(a), Known(b)) = (
                        self.get_value_with_args(a, args.clone()).into_owned(),
                        self.get_value_with_args(b, args.clone()).as_ref(),
                    ) {
                        let return_type = if &a == b {
                            self.get_language_item("True").unwrap()
                        } else {
                            self.get_language_item("False").unwrap()
                        };
                        Known(ConstValue::Value {
                            r#type: return_type,
                            subs: hashmap![],
                        })
                    } else {
                        Unknown
                    }
                }
                Builtin::IsSubtypeOf => {
                    let a = self.dereference(d.get_args()[0]);
                    let b = self.dereference(d.get_args()[1]);
                    if let (
                        Known(ConstValue::Type {
                            r#type: a,
                            arguments: a_args,
                        }),
                        Known(ConstValue::Type {
                            r#type: b,
                            arguments: b_args,
                        }),
                    ) = (
                        self.get_value_with_args(a, args.clone()).into_owned(),
                        self.get_value_with_args(b, args.clone()).as_ref(),
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
                            self.get_language_item("True").unwrap()
                        } else {
                            self.get_language_item("False").unwrap()
                        };
                        Known(ConstValue::Value {
                            r#type: return_type,
                            subs: hashmap![],
                        })
                    } else {
                        Unknown
                    }
                }
                Builtin::IfThenElse => {
                    let true_type = self.get_language_item("True").unwrap();
                    let false_type = self.get_language_item("False").unwrap();
                    let true_result = d.get_args()[2];
                    let false_result = d.get_args()[3];
                    let condition = self.get_value_with_args(d.get_args()[1], args.clone());
                    if condition.as_ref()
                        == &Known(ConstValue::Value {
                            r#type: true_type,
                            subs: hashmap![],
                        })
                    {
                        self.get_value_with_args(true_result, args).into_owned()
                    } else if condition.as_ref()
                        == &Known(ConstValue::Value {
                            r#type: false_type,
                            subs: hashmap![],
                        })
                    {
                        self.get_value_with_args(false_result, args).into_owned()
                    } else {
                        Unknown
                    }
                }
                Builtin::Union => {
                    let a = self.dereference(d.get_args()[0]);
                    let b = self.dereference(d.get_args()[1]);
                    if let (
                        Known(ConstValue::Type {
                            r#type: a,
                            arguments: a_args,
                        }),
                        Known(ConstValue::Type {
                            r#type: b,
                            arguments: b_args,
                        }),
                    ) = (
                        self.get_value_with_args(a, args.clone()).into_owned(),
                        self.get_value_with_args(b, args).into_owned(),
                    ) {
                        let mut args = a_args;
                        args.extend(b_args.into_iter());
                        Known(ConstValue::Type {
                            r#type: a.union(&b),
                            arguments: args,
                        })
                    } else {
                        Unknown
                    }
                }
                Builtin::GodType => Known(ConstValue::Type {
                    r#type: DCompoundType::god_type(),
                    arguments: hashmap![],
                }),
            },
            Def3::DCompoundType(d) => {
                let d = d.clone();
                let params = d.parameters(self);
                let mut const_args = HashMap::new();
                for param in params {
                    if let Some(value) = args.get(&param) {
                        const_args.insert(param, value.clone());
                    } else {
                        return Unknown;
                    }
                }
                Known(ConstValue::Type {
                    r#type: d,
                    arguments: const_args,
                })
            }
            Def3::DConstructor(d) => {
                let type_item = d.r#type();
                let deps = self.get_deps(item);
                let subs: HashMap<_, _> = args
                    .into_iter()
                    .filter(|x| deps.contains(&x.0))
                    .map(|(a, b)| (a.clone(), b.clone()))
                    .collect();
                if subs.len() < deps.len() {
                    Unknown
                } else {
                    Known(ConstValue::Value { r#type: type_item, subs })
                }
            }
            Def3::DMemberAccess(d) => {
                let member_index = d.member_index();
                let base = self
                    .get_value_with_args(d.base(), args.clone())
                    .into_owned();
                if let Known(ConstValue::Value {
                    r#type,
                    subs: values,
                }) = base
                {
                    let Def3::DCompoundType(r#type) = self.get_def3(r#type) else { unreachable!() };
                    assert_eq!(r#type.get_component_types().len(), 1);
                    let (_, r#type) = r#type.get_component_types().iter().next().unwrap();
                    let field = &r#type.get_constructor_parameters()[member_index];
                    let field_constructor = field.1;
                    self.get_value_with_args(field_constructor, values.clone())
                        .into_owned()
                } else {
                    Unknown
                }
            }
            Def3::DOther(d) => self.get_value_with_args(d.0, args).into_owned(),
            Def3::DParameter(d) => {
                if let Some(value) = args.get(d.get_parameter()) {
                    Known(value.clone())
                } else {
                    Unknown
                }
            }
            Def3::DStructLiteral(d) => {
                if d.is_module() {
                    Unknown
                } else {
                    todo!()
                }
            }
            Def3::DSubstitution(d) => {
                let d = d.clone();
                let mut new_args = args.clone();
                for (param, arg) in d.substitutions() {
                    let arg = self.get_value_with_args(*arg, args.clone()).into_owned();
                    if let Known(arg) = arg {
                        new_args.insert(param.ptr_clone(), arg);
                    } else {
                        return Unknown;
                    }
                }
                self.get_value_with_args(d.base(), new_args).into_owned()
            }
            Def3::DInvalid(..) => Unknown,
        }
    }
}

impl Def3 {
    fn add_type_asserts(&self, env: &mut Env) {
        match self {
            Def3::DBuiltin(d) => d.add_type_asserts(env),
            Def3::DCompoundType(..) => {}
            Def3::DConstructor(d) => d.add_type_asserts(env),
            Def3::DMemberAccess(_) => {}
            Def3::DParameter(d) => d.add_type_asserts(env),
            Def3::DSubstitution(d) => d.add_type_asserts(env),
            Def3::DStructLiteral(..) => {}
            Def3::DOther(..) => (),
            Def3::DInvalid(..) => (),
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
