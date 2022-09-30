#[cfg(not(feature = "trace_borrows"))]
use std::cell::RefCell;
use std::{
    fmt::{self, Debug, Formatter},
    rc::Rc,
};

#[cfg(feature = "trace_borrows")]
use debug_cell::RefCell;

use crate::{diagnostic::Position, util::PtrExtension};

pub trait CycleDetectingDebug {
    fn fmt(&self, f: &mut Formatter, stack: &[*const Item]) -> fmt::Result;

    fn to_string(&self, stack: &[*const Item]) -> String {
        let mut string = String::new();
        self.fmt(&mut Formatter::new(&mut string), stack).unwrap();
        string
    }

    fn to_indented_string(&self, stack: &[*const Item], indent_size: u8) -> String {
        let indent = match indent_size {
            0 => "\n",
            1 => "\n   ",
            2 => "\n      ",
            3 => "\n         ",
            _ => panic!("The problem is not with my code, it's with what you're doing with it."),
        };
        self.to_string(stack).replace("\n", indent)
    }
}

pub trait ItemDefinition: CycleDetectingDebug {}

pub struct Item {
    definition: Box<dyn ItemDefinition>,
    position: Option<Position>,
}

pub struct ItemPtr(Rc<RefCell<Item>>);

impl CycleDetectingDebug for ItemPtr {
    fn fmt(&self, f: &mut Formatter, stack: &[*const Item]) -> fmt::Result {
        let ptr = self.0.as_ptr() as *const _;
        if stack.contains(&ptr) {
            write!(f, "@{:?}", ptr)
        } else {
            writeln!(f, "@{:?}", ptr)?;
            let mut new_stack = Vec::from(stack);
            new_stack.push(ptr);
            self.0.borrow().definition.fmt(f, &new_stack)
        }
    }
}

impl Debug for ItemPtr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        CycleDetectingDebug::fmt(self, f, &[])
    }
}

impl ItemPtr {
    pub fn from_definition(def: impl ItemDefinition + 'static) -> Self {
        Self(Rc::new(RefCell::new(Item {
            definition: Box::new(def),
            position: None,
        })))
    }

    pub fn set_position(&self, position: Position) {
        self.0.borrow_mut().position = Some(position);
    }

    pub fn ptr_clone(&self) -> ItemPtr {
        Self(self.0.ptr_clone())
    }
}
