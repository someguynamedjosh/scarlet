use std::{
    cell::RefCell,
    collections::HashSet,
    fmt::{self, Debug, Formatter},
    rc::Rc,
};

use crate::{item::{ItemPtr, dependencies::Dependencies}, util::rcrc};

pub type SetJustification = Vec<StatementJustifications>;
pub type StatementJustifications = Vec<StatementJustification>;
pub type StatementJustification = Vec<InvariantSetPtr>;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct JustificationRequirement {
    pub statement: ItemPtr,
    pub allowed_dependencies: Dependencies,
}

#[derive(Clone, PartialEq, Eq)]
pub struct InvariantSet {
    pub(super) context: ItemPtr,
    pub(super) statements: Vec<ItemPtr>,
    /// For the original statements to hold, all the statements in this list
    /// must also hold.
    pub(super) justification_requirements: Vec<JustificationRequirement>,
    pub(super) set_justification: Option<SetJustification>,
    pub(super) connected_to_root: bool,
    pub(super) required: bool,
    pub(super) dependencies: HashSet<ItemPtr>,
}

impl Debug for InvariantSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("InvariantSet")
            .field("context", &self.context.debug_label())
            .field("statements", &self.statements)
            .field(
                "justification_requirements",
                &self.justification_requirements,
            )
            .field("required", &self.required)
            .field("connected_to_root", &self.connected_to_root)
            .field("dependencies", &self.dependencies)
            .finish_non_exhaustive()
    }
}

pub type InvariantSetPtr = Rc<RefCell<InvariantSet>>;

impl InvariantSet {
    pub fn new_empty(context: ItemPtr) -> InvariantSetPtr {
        Self::new(context, vec![], vec![], HashSet::new())
    }

    pub fn new(
        context: ItemPtr,
        statements: Vec<ItemPtr>,
        justification_requirements: Vec<JustificationRequirement>,
        dependencies: HashSet<ItemPtr>,
    ) -> InvariantSetPtr {
        rcrc(Self {
            context,
            statements,
            justification_requirements,
            set_justification: None,
            connected_to_root: false,
            required: true,
            dependencies,
        })
    }

    pub fn new_not_required(
        context: ItemPtr,
        statements: Vec<ItemPtr>,
        justification_requirements: Vec<JustificationRequirement>,
        dependencies: HashSet<ItemPtr>,
    ) -> InvariantSetPtr {
        rcrc(Self {
            context,
            statements,
            justification_requirements,
            set_justification: None,
            connected_to_root: false,
            required: false,
            dependencies,
        })
    }

    pub(crate) fn new_justified_by(
        context: ItemPtr,
        statements: Vec<ItemPtr>,
        justified_by: SetJustification,
    ) -> InvariantSetPtr {
        rcrc(Self {
            context,
            statements,
            justification_requirements: Vec::new(),
            set_justification: Some(justified_by),
            connected_to_root: false,
            required: false,
            dependencies: HashSet::new(),
        })
    }

    pub(super) fn new_recursive_justification(
        context: ItemPtr,
        dependencies: HashSet<ItemPtr>,
    ) -> InvariantSetPtr {
        rcrc(Self {
            context,
            statements: Vec::new(),
            justification_requirements: Vec::new(),
            set_justification: None,
            connected_to_root: true,
            required: false,
            dependencies,
        })
    }

    pub fn new_root_statements_depending_on(
        context: ItemPtr,
        statements: Vec<ItemPtr>,
        dependencies: HashSet<ItemPtr>,
    ) -> InvariantSetPtr {
        rcrc(Self {
            context,
            statements,
            justification_requirements: Vec::new(),
            set_justification: None,
            connected_to_root: true,
            required: true,
            dependencies,
        })
    }

    /// Get a reference to the invariant set's statements.
    #[must_use]
    pub fn statements(&self) -> &[ItemPtr] {
        self.statements.as_ref()
    }

    /// Get a reference to the invariant set's justification requirements.
    #[must_use]
    pub fn justification_requirements(&self) -> &[JustificationRequirement] {
        self.justification_requirements.as_ref()
    }

    /// Get a reference to the invariant set's justified by.
    #[must_use]
    pub fn justified_by(&self) -> Option<&SetJustification> {
        self.set_justification.as_ref()
    }

    /// Get a reference to the invariant set's dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &HashSet<ItemPtr> {
        &self.dependencies
    }

    pub fn push(&mut self, statement: ItemPtr) {
        self.statements.push(statement);
    }
}
