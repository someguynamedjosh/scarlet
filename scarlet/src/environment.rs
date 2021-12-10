pub mod dependencies;
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
    shared::Pool,
    tokens::structure::Token,
};

pub const BUILTIN_ITEM_NAMES: &[&str] = &["true", "false", "void", "Bool"];

#[derive(Debug)]
pub struct Environment<'x> {
    builtin_items: HashMap<&'static str, ConstructId>,
    pub(crate) constructs: ConstructPool<'x>,
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
            uniques: Pool::new(),
            variables: Pool::new(),
        };
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
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Placeholder,
            parent_scope: None,
        };
        self.constructs.push(con)
    }

    pub fn push_construct(&mut self, construct: BoxedConstruct) -> ConstructId {
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
            parent_scope: None,
        };
        self.constructs.push(con)
    }

    pub fn push_unique(&mut self) -> UniqueId {
        self.uniques.push(Unique)
    }

    pub fn push_unresolved(
        &mut self,
        token: Token<'x>,
        parent_scope: Option<ConstructId>,
    ) -> ConstructId {
        if let Token::Construct(con) = token {
            con
        } else {
            let con = AnnotatedConstruct {
                definition: ConstructDefinition::Unresolved(token),
                parent_scope,
            };
            self.constructs.push(con)
        }
    }

    pub fn push_variable(&mut self, invariants: Vec<ConstructId>, capturing: bool) -> ConstructId {
        let id = self.variables.push(Variable);
        let def = CVariable {
            capturing,
            id,
            invariants,
        };
        self.push_construct(Box::new(def))
    }

    pub(crate) fn check(&mut self, con_id: ConstructId) {
        let con = self.get_construct(con_id).dyn_clone();
        con.check(self);
    }
}
