use std::{
    any::Any,
    collections::HashSet,
    fmt::{self, Debug, Formatter},
    hash::Hash,
    rc::Rc,
};

use debug_cell::RefCell;

use crate::{
    definitions::{
        builtin::DBuiltin, compound_type::DCompoundType, hole::DHole, identifier::DIdentifier,
        member_access::DMemberAccess, new_value::DNewValue, parameter::DParameter,
        struct_literal::DStructLiteral, unresolved_substitution::DUnresolvedSubstitution,
    },
    diagnostic::Position,
    util::PtrExtension,
};

pub struct Item<Definition, Analysis> {
    pub definition: Definition,
    pub analysis: Analysis,
}

pub struct ItemRef<Definition, Analysis> {
    pub item: Rc<RefCell<Item<Definition, Analysis>>>,
    pub position: Position,
}

impl<Definition, Analysis> PartialEq for ItemRef<Definition, Analysis> {
    fn eq(&self, other: &Self) -> bool {
        self.item.is_same_instance_as(&other.item)
    }
}

impl<Definition, Analysis> Eq for ItemRef<Definition, Analysis> {}

impl<Definition, Analysis> Hash for ItemRef<Definition, Analysis> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.item.as_ptr().to_bits().hash(state)
    }
}

impl<Definition, Analysis> Clone for ItemRef<Definition, Analysis> {
    fn clone(&self) -> Self {
        self.ptr_clone()
    }
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis> Debug
    for ItemRef<Definition, Analysis>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        CycleDetectingDebug::fmt(
            &self.item.borrow().definition,
            f,
            &mut CddContext {
                recursed_on: &mut HashSet::new(),
                stack: &[],
            },
        )
    }
}

pub struct CddContext<'a, 'b> {
    stack: &'a [*const ()],
    recursed_on: &'b mut HashSet<*const ()>,
}

pub trait CycleDetectingDebug {
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result;

    fn to_string(&self, ctx: &mut CddContext) -> String {
        let mut string = String::new();
        self.fmt(&mut Formatter::new(&mut string), ctx).unwrap();
        string
    }

    fn to_indented_string(&self, ctx: &mut CddContext, indent_size: u8) -> String {
        let mut result = self.to_string(ctx);
        for _ in 0..indent_size {
            result = result.replace("\n", "\n   ");
        }
        result
    }
}

pub trait NamedAny: Any {
    fn type_name<'a>(&'a self) -> &'static str;
}

impl<T: Any> NamedAny for T {
    fn type_name<'a>(&'a self) -> &'static str {
        std::any::type_name::<T>()
    }
}

pub trait ItemDefinition<Definition: ItemDefinition<Definition, Analysis>, Analysis>:
    CycleDetectingDebug
{
    fn children(&self) -> Vec<ItemRef<Definition, Analysis>>;
}

impl<Definition: ItemDefinition<Definition, Analysis>, Analysis> CycleDetectingDebug
    for ItemRef<Definition, Analysis>
{
    fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
        todo!()
    }
}

impl<Definition, Analysis> ItemRef<Definition, Analysis> {
    pub fn ptr_clone(&self) -> Self {
        Self {
            item: Rc::clone(&self.item),
            position: self.position,
        }
    }

    pub(crate) fn get_position(&self) -> Position {
        self.position
    }
}

impl<Definition> ItemRef<Definition, ()> {
    fn new(definition: Definition, position: Position) -> Self {
        ItemRef {
            item: Rc::new(RefCell::new(Item {
                definition,
                analysis: (),
            })),
            position,
        }
    }
}

macro_rules! definition_enum {
    ($Name:ident, $Analysis:ty, { $($Subtype:ident),* }) => {
        pub enum $Name {
            $($Subtype($Subtype<$Name, $Analysis>)),*
        }

        impl CycleDetectingDebug for $Name {
            fn fmt(&self, f: &mut Formatter, ctx: &mut CddContext) -> fmt::Result {
                match self {
                    $(Self::$Subtype(def) => CycleDetectingDebug::fmt(def, f, ctx)),*
                }
            }
        }

        $(impl From<$Subtype<$Name, $Analysis>> for $Name {
            fn from(_: $Subtype<$Name, $Analysis>) -> Self {
                todo!()
            }
        })*
    }
}

definition_enum!(DeUnresolved, (), {
    DBuiltin, DCompoundType, DIdentifier, DMemberAccess, DParameter, DStructLiteral, DUnresolvedSubstitution
});

impl ItemDefinition<DeUnresolved, ()> for DeUnresolved {
    fn children(&self) -> Vec<ItemRef<DeUnresolved, ()>> {
        todo!()
    }
}

pub trait IntoRef<Definition> {
    fn into_ref(self, position: Position) -> ItemRef<Definition, ()>;
}

impl<Definition, Source: Into<Definition>> IntoRef<Definition> for Source {
    fn into_ref(self, position: Position) -> ItemRef<Definition, ()> {
        ItemRef::new(self.into(), position)
    }
}
