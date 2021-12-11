pub mod dependencies;
mod reduce;
pub mod resolve;
pub mod substitute;
pub mod util;
mod vomit;

use std::collections::HashMap;

use crate::{
    constructs::{
        base::{
            AnnotatedConstruct, BoxedConstruct, ConstructDefinition, ConstructId, ConstructPool,
        },
        unique::{Unique, UniqueId, UniquePool},
        variable::{CVariable, Variable, VariablePool},
    },
    scope::{AnnotatedScope, SEmpty, SRoot, Scope, ScopeId, ScopePool},
    shared::{Pool, TripleBool},
    tokens::structure::Token,
};

pub const BUILTIN_ITEM_NAMES: &[&str] = &["true", "false", "void"];

#[derive(Debug)]
pub struct Environment<'x> {
    builtin_items: HashMap<&'static str, ConstructId>,
    pub(crate) constructs: ConstructPool<'x>,
    pub(crate) scopes: ScopePool,
    pub(crate) uniques: UniquePool,
    pub(crate) variables: VariablePool,
}

impl<'x> Environment<'x>
where
    ConstructId: 'static,
{
    pub fn new() -> Self {
        let mut this = Self {
            builtin_items: HashMap::new(),
            constructs: Pool::new(),
            scopes: Pool::new(),
            uniques: Pool::new(),
            variables: Pool::new(),
        };
        this.push_scope(Box::new(SRoot), None);
        for &name in BUILTIN_ITEM_NAMES {
            let id = this.push_placeholder();
            this.builtin_items.insert(name, id);
        }
        this
    }

    pub fn define_builtin_item(&mut self, name: &str, definition: ConstructId) {
        let id = self.get_builtin_item(name);
        self.constructs[id].definition =
            ConstructDefinition::Unresolved(Token::Construct(definition));
    }

    pub fn get_builtin_item(&self, name: &str) -> ConstructId {
        *self
            .builtin_items
            .get(name)
            .unwrap_or_else(|| todo!("nice error, no builtin item named {}", name))
    }

    pub fn push_placeholder(&mut self) -> ConstructId {
        let scope = self.new_empty_scope();
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Placeholder,
            scope,
        };
        self.constructs.push(con)
    }

    pub fn push_construct(
        &mut self,
        construct: BoxedConstruct,
        containing: Vec<ConstructId>,
    ) -> ConstructId {
        let scope = self.new_empty_scope();
        for con in containing {
            let child = self.constructs[con].scope;
            self.scopes[child].parent = Some(scope);
        }
        for (id, acon) in &self.constructs {
            if acon
                .definition
                .as_resolved()
                .map(|con| con.eq(&*construct))
                .unwrap_or(false)
            {
                return id;
            }
        }
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Resolved(construct),
            scope,
        };
        self.constructs.push(con)
    }

    pub fn get_construct_scope(&mut self, con_id: ConstructId) -> ScopeId {
        self.constructs[con_id].scope
    }

    pub fn push_scope(&mut self, scope: Box<dyn Scope>, parent: Option<ScopeId>) -> ScopeId {
        self.scopes.push(AnnotatedScope { scope, parent })
    }

    pub fn change_scope(&mut self, id: ScopeId, scope: Box<dyn Scope>) {
        self.scopes[id].scope = scope;
    }

    pub fn set_scope_parent(&mut self, id: ScopeId, new_parent: ScopeId) {
        self.scopes[id].parent = Some(new_parent);
    }

    pub fn get_scope(&self, id: ScopeId) -> &AnnotatedScope {
        &self.scopes[id]
    }

    pub fn root_scope(&self) -> ScopeId {
        self.scopes.first().unwrap()
    }

    pub fn new_empty_scope(&mut self) -> ScopeId {
        self.push_scope(Box::new(SEmpty), None)
    }

    pub fn push_unique(&mut self) -> UniqueId {
        self.uniques.push(Unique)
    }

    pub fn push_unresolved(&mut self, token: Token<'x>, scope: ScopeId) -> ConstructId {
        if let Token::Construct(con) = token {
            con
        } else {
            let con = AnnotatedConstruct {
                definition: ConstructDefinition::Unresolved(token),
                scope,
            };
            self.constructs.push(con)
        }
    }

    pub fn push_variable(&mut self, invariants: ConstructId, capturing: bool) -> ConstructId {
        let id = self.variables.push(Variable);
        let def = CVariable {
            capturing,
            id,
            invariants,
        };
        self.push_construct(Box::new(def), vec![invariants])
    }

    pub(crate) fn check(&mut self, con_id: ConstructId) {
        let con = self.get_construct(con_id).dyn_clone();
        con.check(self);
    }

    pub(crate) fn is_def_equal(&mut self, left: ConstructId, right: ConstructId) -> TripleBool {
        let other = self.get_construct(right).dyn_clone();
        self.get_construct(left)
            .dyn_clone()
            .is_def_equal(self, &*other)
    }
}
