pub mod dependencies;
pub mod discover_equality;
pub mod from;
pub mod invariants;
pub mod overlay;
pub mod path;
mod reduce;
pub mod resolve;
pub mod sub_expr;
mod test_util;
pub mod util;
mod vomit;

use std::{collections::HashMap, ops::ControlFlow};

use self::{dependencies::DepResStack, resolve::ResolveStack};
use crate::{
    constructs::{
        base::{AnnotatedConstruct, ConstructDefinition, ConstructId, ConstructPool},
        downcast_construct,
        substitution::{CSubstitution, Substitutions},
        unique::{Unique, UniqueId, UniquePool},
        variable::{CVariable, Variable, VariableId, VariablePool},
        Construct,
    },
    resolvable::{BoxedResolvable, RPlaceholder, Resolvable},
    scope::{SRoot, Scope},
    shared::Pool,
};

#[cfg(not(feature = "no_axioms"))]
pub const LANGUAGE_ITEM_NAMES: &[&str] = &[
    "true",
    "false",
    "void",
    "x",
    "and",
    "t_trivial_statement",
    "t_invariant_truth_statement",
    "t_invariant_truth_rev_statement",
    "t_eq_ext_rev_statement",
    "t_inv_eq_statement",
    "t_refl_statement",
    "t_decision_eq_statement",
    "t_decision_neq_statement",
];

#[cfg(feature = "no_axioms")]
pub const LANGUAGE_ITEM_NAMES: &[&str] = &["true", "false", "void"];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UnresolvedConstructError(pub ConstructId);

pub type CheckResult = Result<(), UnresolvedConstructError>;

#[derive(Debug)]
pub struct Environment<'x> {
    language_items: HashMap<&'static str, ConstructId>,
    pub(crate) constructs: ConstructPool<'x>,
    pub(crate) uniques: UniquePool,
    pub(crate) variables: VariablePool,
    pub(super) dep_res_stack: DepResStack,
    pub(super) resolve_stack: ResolveStack,
    // pub(super) def_equal_memo_table: HashMap<DefEqualQuery, DefEqualResult>,
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
            resolve_stack: ResolveStack::new(),
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
        let var_id = downcast_construct::<CVariable>(&*definition).map(CVariable::get_id);
        if let Some(var_id) = var_id {
            self.variables[var_id].construct = Some(construct);
        }
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
            from_dex: None,
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
            from_dex: None,
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
        let var_id = downcast_construct::<CVariable>(&*construct).map(CVariable::get_id);
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Resolved(construct),
            reduced: ConstructDefinition::Unresolved(Box::new(RPlaceholder)),
            invariants: None,
            scope,
            from_dex: None,
        };
        let id = self.constructs.push(con);
        if let Some(var_id) = var_id {
            self.variables[var_id].construct = Some(id);
        }
        id
    }

    pub fn push_other(&mut self, other: ConstructId, scope: Box<dyn Scope>) -> ConstructId {
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Other(other),
            reduced: ConstructDefinition::Unresolved(Box::new(RPlaceholder)),
            invariants: None,
            scope,
            from_dex: None,
        };
        self.constructs.push(con)
    }

    pub fn push_unique(&mut self) -> UniqueId {
        self.uniques.push(Unique)
    }

    pub fn push_variable(&mut self, var: Variable) -> VariableId {
        let id = self.variables.push(var);
        self.variables[id].id = Some(id);
        id
    }

    pub fn get_variable(&self, id: VariableId) -> &Variable {
        &self.variables[id]
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
            from_dex: None,
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

    pub(crate) fn check(&mut self, con_id: ConstructId) -> CheckResult {
        let con = self.get_construct_definition(con_id)?.dyn_clone();
        let scope = self.get_construct_scope(con_id).dyn_clone();
        con.check(self, con_id, scope)
    }

    pub(crate) fn check_all(&mut self) -> CheckResult {
        match self.for_each_construct(|env, id| match env.check(id) {
            Ok(ok) => ControlFlow::Continue(ok),
            Err(err) => ControlFlow::Break(err),
        }) {
            None => Ok(()),
            Some(err) => Err(err),
        }
    }

    pub(crate) fn substitute(
        &mut self,
        base: ConstructId,
        substitutions: &Substitutions,
    ) -> ConstructId {
        if substitutions.len() == 0 {
            base
        } else {
            let con = CSubstitution::new_unchecked(base, substitutions.clone());
            let scope = self.constructs[base].scope.dyn_clone();
            self.push_construct(con, scope)
        }
    }

    pub(crate) fn language_item_names(&self) -> impl Iterator<Item = &'static str> {
        LANGUAGE_ITEM_NAMES.iter().copied()
    }
}
