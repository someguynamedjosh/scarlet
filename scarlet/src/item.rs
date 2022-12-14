use std::{rc::Rc, collections::HashSet, fmt::{Formatter, self}, any::Any};

use debug_cell::RefCell;

use crate::diagnostic::Position;

pub struct Item<Definition, Analysis> {
    pub definition: Definition,
    pub analysis: Analysis,
}

pub struct ItemRef<Definition, Analysis> {
    pub item: Rc<RefCell<Item<Definition, Analysis>>>,
    pub position: Position,
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

pub trait ItemDefinition<Definition: ItemDefinition<Definition, Analysis>, Analysis> : CycleDetectingDebug {
    fn children(&self) -> Vec<ItemRef<Definition, Analysis>>;
}
