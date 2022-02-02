#[cfg(test)]
mod def_equal_tests;
pub mod dependencies;
pub mod overlay;
pub mod path;
mod reduce;
pub mod resolve;
pub mod util;
mod vomit;

use std::{collections::HashMap, ops::ControlFlow};

use self::{dependencies::DepResStack, util::InvariantStack};
use crate::{
    constructs::{
        base::{AnnotatedConstruct, ConstructDefinition, ConstructId, ConstructPool},
        substitution::{SubExpr, Substitutions, CSubstitution},
        unique::{Unique, UniqueId, UniquePool},
        variable::{Variable, VariableId, VariablePool},
        Construct,
    },
    resolvable::{BoxedResolvable, RPlaceholder, Resolvable},
    scope::{SRoot, Scope},
    shared::{Pool, TripleBool},
};

#[cfg(not(feature = "no_axioms"))]
pub const LANGUAGE_ITEM_NAMES: &[&str] = &[
    "true",
    "false",
    "void",
    "t_trivial_statement",
    "t_invariant_truth_statement",
    "t_invariant_truth_rev_statement",
    "t_eq_ext_rev_statement",
    "t_inv_eq_statement",
];

#[cfg(feature = "no_axioms")]
pub const LANGUAGE_ITEM_NAMES: &[&str] = &["true", "false", "void"];

#[derive(Debug)]
pub struct Environment<'x> {
    language_items: HashMap<&'static str, ConstructId>,
    pub(crate) constructs: ConstructPool<'x>,
    pub(crate) uniques: UniquePool,
    pub(crate) variables: VariablePool,
    pub(super) dep_res_stack: DepResStack,
    pub(super) invariant_stack: InvariantStack,
    use_reduced_definitions_while_vomiting: bool,
}

impl<'x> Environment<'x> {
    pub fn new() -> Self {
        let mut this = Self {
            language_items: HashMap::new(),
            constructs: Pool::new(),
            uniques: Pool::new(),
            variables: Pool::new(),
            dep_res_stack: DepResStack::new(),
            invariant_stack: InvariantStack::new(),
            use_reduced_definitions_while_vomiting: true,
        };
        for &name in LANGUAGE_ITEM_NAMES {
            let id = this.push_placeholder(Box::new(SRoot));
            this.language_items.insert(name, id);
        }
        this
    }

    pub fn define_language_item(&mut self, name: &str, definition: ConstructId) {
        let id = self.get_language_item(name);
        self.constructs[id].definition = definition.into();
    }

    pub fn define_construct(&mut self, construct: ConstructId, definition: impl Construct) {
        self.define_dyn_construct(construct, Box::new(definition))
    }

    pub fn define_dyn_construct(&mut self, construct: ConstructId, definition: Box<dyn Construct>) {
        self.constructs[construct].definition = definition.into();
    }

    pub fn define_unresolved(
        &mut self,
        construct: ConstructId,
        definition: impl Resolvable<'x> + 'x,
    ) {
        self.constructs[construct].definition =
            ConstructDefinition::Unresolved(Box::new(definition));
    }

    pub fn get_language_item(&self, name: &str) -> ConstructId {
        *self
            .language_items
            .get(name)
            .unwrap_or_else(|| todo!("nice error, no language item named {}", name))
    }

    pub fn push_placeholder(&mut self, scope: Box<dyn Scope>) -> ConstructId {
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Unresolved(Box::new(RPlaceholder)),
            reduced: ConstructDefinition::Unresolved(Box::new(RPlaceholder)),
            invariants: None,
            scope,
        };
        self.constructs.push(con)
    }

    pub fn push_scope(&mut self, scope: Box<dyn Scope>) -> ConstructId {
        let void = self.get_language_item("void");
        self.constructs.push(AnnotatedConstruct {
            definition: ConstructDefinition::Other(void),
            reduced: ConstructDefinition::Unresolved(Box::new(RPlaceholder)),
            invariants: None,
            scope,
        })
    }

    pub fn push_construct(
        &mut self,
        construct: impl Construct,
        scope: Box<dyn Scope>,
    ) -> ConstructId {
        self.push_dyn_construct(Box::new(construct), scope)
    }

    pub fn push_dyn_construct(
        &mut self,
        construct: Box<dyn Construct>,
        scope: Box<dyn Scope>,
    ) -> ConstructId {
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Resolved(construct),
            reduced: ConstructDefinition::Unresolved(Box::new(RPlaceholder)),
            invariants: None,
            scope,
        };
        self.constructs.push(con)
    }

    pub fn push_other(&mut self, other: ConstructId, scope: Box<dyn Scope>) -> ConstructId {
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Other(other),
            reduced: ConstructDefinition::Unresolved(Box::new(RPlaceholder)),
            invariants: None,
            scope,
        };
        self.constructs.push(con)
    }

    pub fn push_unique(&mut self) -> UniqueId {
        self.uniques.push(Unique)
    }

    pub fn push_variable(&mut self) -> VariableId {
        self.variables.push(Variable)
    }

    pub fn push_unresolved(
        &mut self,
        definition: impl Resolvable<'x> + 'x,
        scope: Box<dyn Scope>,
    ) -> ConstructId {
        self.push_dyn_unresolved(Box::new(definition), scope)
    }

    pub fn push_dyn_unresolved(
        &mut self,
        definition: BoxedResolvable<'x>,
        scope: Box<dyn Scope>,
    ) -> ConstructId {
        self.constructs.push(AnnotatedConstruct {
            definition: ConstructDefinition::Unresolved(definition),
            reduced: ConstructDefinition::Unresolved(Box::new(RPlaceholder)),
            invariants: None,
            scope,
        })
    }

    pub fn for_each_construct_returning_nothing(
        &mut self,
        mut visitor: impl FnMut(&mut Self, ConstructId) -> (),
    ) {
        let mut next_id = self.constructs.first();
        while let Some(id) = next_id {
            visitor(self, id);
            next_id = self.constructs.next(id);
        }
    }

    pub fn for_each_construct<T>(
        &mut self,
        mut visitor: impl FnMut(&mut Self, ConstructId) -> ControlFlow<T>,
    ) -> Option<T> {
        let mut next_id = self.constructs.first();
        while let Some(id) = next_id {
            match visitor(self, id) {
                ControlFlow::Continue(()) => (),
                ControlFlow::Break(value) => return Some(value),
            }
            next_id = self.constructs.next(id);
        }
        None
    }

    pub(crate) fn check(&mut self, con_id: ConstructId) {
        let con = self.get_construct_definition(con_id).dyn_clone();
        let scope = self.get_construct_scope(con_id).dyn_clone();
        con.check(self, con_id, scope);
    }

    pub(crate) fn check_all(&mut self) {
        self.for_each_construct_returning_nothing(Self::check);
    }

    pub(crate) fn is_def_equal(&mut self, left: SubExpr, right: SubExpr) -> TripleBool {
        let result = self
            .get_construct_definition(left.0)
            .dyn_clone()
            .is_def_equal(self, &left.1, right);
        if result == TripleBool::Unknown {
            self.get_construct_definition(right.0)
                .dyn_clone()
                .is_def_equal(self, &right.1, left)
        } else {
            result
        }
    }

    pub(crate) fn is_def_equal_without_subs(
        &mut self,
        left: ConstructId,
        right: ConstructId,
    ) -> TripleBool {
        self.is_def_equal(
            SubExpr(left, &Default::default()),
            SubExpr(right, &Default::default()),
        )
    }

    pub(crate) fn substitute(&mut self, base: ConstructId, substitutions: &Substitutions) -> ConstructId {
        let con = CSubstitution::new_unchecked(base, substitutions.clone());
        let scope = self.constructs[base].scope.dyn_clone();
        self.push_construct(con, scope)
    }
}
