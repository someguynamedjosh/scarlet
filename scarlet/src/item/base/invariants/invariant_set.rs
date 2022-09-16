use std::{
    cell::RefCell,
    collections::HashSet,
    fmt::{self, Debug, Formatter},
    rc::Rc,
};

use crate::{
    item::{dependencies::Dependencies, ItemPtr},
    util::rcrc,
};

#[derive(Clone, PartialEq, Eq)]
pub struct InvariantSet {
    pub(super) context: ItemPtr,
    pub(super) statements: Vec<ItemPtr>,
}

impl Debug for InvariantSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("InvariantSet")
            .field("context", &self.context.debug_label())
            .field("statements", &self.statements)
            .finish_non_exhaustive()
    }
}

pub type InvariantSetPtr = Rc<RefCell<InvariantSet>>;

impl InvariantSet {
    pub fn new_empty(context: ItemPtr) -> InvariantSetPtr {
        Self::new(context, vec![])
    }

    pub fn new(context: ItemPtr, statements: Vec<ItemPtr>) -> InvariantSetPtr {
        rcrc(Self {
            context,
            statements,
        })
    }

    /// Get a reference to the invariant set's statements.
    #[must_use]
    pub fn statements(&self) -> &[ItemPtr] {
        self.statements.as_ref()
    }

    pub fn push(&mut self, statement: ItemPtr) {
        self.statements.push(statement);
    }
}
