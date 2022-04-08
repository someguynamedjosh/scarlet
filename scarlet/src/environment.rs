pub mod dependencies;
pub mod discover_equality;
pub mod from;
pub mod invariants;
pub mod overlay;
pub mod recursion;
pub mod reduce;
pub mod resolve;
pub mod sub_expr;
pub mod test_util;
pub mod util;
pub mod vomit;

use std::{collections::HashMap, ops::ControlFlow};

use self::{
    dependencies::DepResStack,
    invariants::{justify::JustifyStack, InvariantSetPool},
    resolve::ResolveStack,
};
use crate::{
    constructs::{
        base::{Item, ItemDefinition, ItemId, ItemPool},
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
pub const LANGUAGE_ITEM_NAMES: &[&str] = &["true", "false", "void", "x", "and"];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UnresolvedItemError(pub ItemId);

pub type CheckResult = Result<(), UnresolvedItemError>;

#[derive(Debug)]
pub struct Environment<'x> {
    language_items: HashMap<&'static str, ItemId>,
    pub(crate) items: ItemPool<'x>,
    pub(crate) invariant_sets: InvariantSetPool,
    pub(crate) uniques: UniquePool,
    pub(crate) variables: VariablePool,
    pub(super) dep_res_stack: DepResStack,
    pub(super) resolve_stack: ResolveStack,
    pub(super) justify_stack: JustifyStack,
    pub(super) auto_theorems: Vec<ItemId>,
    // pub(super) def_equal_memo_table: HashMap<DefEqualQuery, DefEqualResult>,
}

impl<'x> Environment<'x> {
    pub fn new() -> Self {
        let mut this = Self {
            language_items: HashMap::new(),
            items: Pool::new(),
            invariant_sets: Pool::new(),
            uniques: Pool::new(),
            variables: Pool::new(),
            dep_res_stack: DepResStack::new(),
            resolve_stack: ResolveStack::new(),
            justify_stack: JustifyStack::new(),
            auto_theorems: Vec::new(),
        };
        for &name in LANGUAGE_ITEM_NAMES {
            let id = this.push_placeholder(Box::new(SRoot));
            this.language_items.insert(name, id);
        }
        this
    }

    pub fn define_language_item(&mut self, name: &str, definition: ItemId) {
        let id = self.get_language_item(name);
        self.items[id].definition = definition.into();
    }

    pub fn define_item(&mut self, item: ItemId, definition: impl Construct) {
        self.define_dyn_item(item, Box::new(definition))
    }

    pub fn define_dyn_item(&mut self, item: ItemId, definition: Box<dyn Construct>) {
        let var_id = downcast_construct::<CVariable>(&*definition).map(CVariable::get_id);
        if let Some(var_id) = var_id {
            self.variables[var_id].item = Some(item);
        }
        self.items[item].definition = definition.into();
    }

    pub fn define_unresolved(&mut self, item: ItemId, definition: impl Resolvable<'x> + 'x) {
        self.items[item].unresolved = Some(Box::new(definition));
    }

    #[track_caller]
    pub fn get_language_item(&self, name: &str) -> ItemId {
        *self
            .language_items
            .get(name)
            .expect(&format!("nice error, no language item named {}", name))
    }

    pub fn push_placeholder(&mut self, scope: Box<dyn Scope>) -> ItemId {
        let item = Item {
            definition: ItemDefinition::Placeholder,
            reduced: ItemDefinition::Placeholder,
            unresolved: None,
            invariants: None,
            scope,
            from_dex: None,
            name: None,
        };
        self.items.push(item)
    }

    pub fn push_scope(&mut self, scope: Box<dyn Scope>) -> ItemId {
        let void = self.get_language_item("void");
        self.items.push(Item {
            definition: ItemDefinition::Other(void),
            reduced: ItemDefinition::Placeholder,
            unresolved: None,
            invariants: None,
            scope,
            from_dex: None,
            name: None,
        })
    }

    pub fn push_construct(&mut self, construct: impl Construct, scope: Box<dyn Scope>) -> ItemId {
        self.push_dyn_construct(Box::new(construct), scope)
    }

    pub fn push_dyn_construct(
        &mut self,
        construct: Box<dyn Construct>,
        scope: Box<dyn Scope>,
    ) -> ItemId {
        let var_id = downcast_construct::<CVariable>(&*construct).map(CVariable::get_id);
        let item = Item {
            definition: ItemDefinition::Resolved(construct),
            reduced: ItemDefinition::Placeholder,
            unresolved: None,
            invariants: None,
            scope,
            from_dex: None,
            name: None,
        };
        let id = self.items.push(item);
        if let Some(var_id) = var_id {
            self.variables[var_id].item = Some(id);
        }
        id
    }

    pub fn push_other(&mut self, other: ItemId, scope: Box<dyn Scope>) -> ItemId {
        let item = Item {
            definition: ItemDefinition::Other(other),
            reduced: ItemDefinition::Placeholder,
            unresolved: None,
            invariants: None,
            scope,
            from_dex: None,
            name: None,
        };
        let id = self.items.push(item);
        self.arrest_recursion(id);
        id
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
    ) -> ItemId {
        self.push_dyn_unresolved(Box::new(definition), scope)
    }

    pub fn push_dyn_unresolved(
        &mut self,
        definition: BoxedResolvable<'x>,
        scope: Box<dyn Scope>,
    ) -> ItemId {
        self.items.push(Item {
            definition: ItemDefinition::Placeholder,
            reduced: ItemDefinition::Placeholder,
            unresolved: Some(definition),
            invariants: None,
            scope,
            from_dex: None,
            name: None,
        })
    }

    pub fn for_each_item_returning_nothing(
        &mut self,
        mut visitor: impl FnMut(&mut Self, ItemId) -> (),
    ) {
        let mut next_id = self.items.first();
        while let Some(id) = next_id {
            visitor(self, id);
            next_id = self.items.next(id);
        }
    }

    pub fn for_each_item<T>(
        &mut self,
        mut visitor: impl FnMut(&mut Self, ItemId) -> ControlFlow<T>,
    ) -> Option<T> {
        let mut next_id = self.items.first();
        while let Some(id) = next_id {
            match visitor(self, id) {
                ControlFlow::Continue(()) => (),
                ControlFlow::Break(value) => return Some(value),
            }
            next_id = self.items.next(id);
        }
        None
    }

    pub(crate) fn check(&mut self, item_id: ItemId) -> CheckResult {
        let item = self.get_item_as_construct(item_id)?.dyn_clone();
        let scope = self.get_item_scope(item_id).dyn_clone();
        item.check(self, item_id, scope)
    }

    pub(crate) fn check_all(&mut self) -> CheckResult {
        let mut encountered_err = false;
        for limit in 0..16 {
            let mut maybe_id = self.invariant_sets.first();
            while let Some(id) = maybe_id {
                let res = self.justify(id, limit);
                if limit == 15 {
                    if let Err(err) = res {
                        eprintln!("Error while justifying invariant set:");
                        eprintln!("{:?}", err);
                        encountered_err = true;
                    }
                }
                maybe_id = self.invariant_sets.next(id);
            }
        }
        if encountered_err {
            todo!("nice error: Invariants are not justified.");
        }
        match self.for_each_item(|env, id| match env.check(id) {
            Ok(ok) => ControlFlow::Continue(ok),
            Err(err) => panic!("{:?}", err),
        }) {
            None => Ok(()),
            Some(err) => Err(err),
        }
    }

    pub(crate) fn substitute_unchecked(
        &mut self,
        base: ItemId,
        substitutions: &Substitutions,
    ) -> ItemId {
        if substitutions.len() == 0 {
            base
        } else {
            let con = CSubstitution::new_unchecked(self, base, substitutions.clone());
            let scope = self.items[base].scope.dyn_clone();
            self.push_construct(con, scope)
        }
    }

    pub(crate) fn language_item_names(&self) -> impl Iterator<Item = &'static str> {
        LANGUAGE_ITEM_NAMES.iter().copied()
    }
}
