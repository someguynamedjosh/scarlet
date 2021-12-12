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
        variable::VariablePool,
        Construct,
    },
    scope::{SPlaceholder, SPlain, SRoot, Scope},
    shared::{Pool, TripleBool},
    tokens::structure::Token,
};

pub const BUILTIN_ITEM_NAMES: &[&str] = &["true", "false", "void"];

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
        let root = this.constructs.push(AnnotatedConstruct {
            definition: ConstructDefinition::Placeholder,
            scope: Box::new(SRoot),
        });
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
        let scope = Box::new(SPlaceholder);
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Placeholder,
            scope,
        };
        self.constructs.push(con)
    }

    pub fn define_placeholder(
        &mut self,
        con: ConstructId,
        def: impl Construct + 'static,
        scope: impl Scope + 'static,
    ) {
        let definition = ConstructDefinition::Resolved(Box::new(def));
        let scope = Box::new(scope);
        self.constructs[con] = AnnotatedConstruct { definition, scope }
    }

    pub fn push_construct(
        &mut self,
        construct: BoxedConstruct,
        scope: impl Scope + 'static,
    ) -> ConstructId {
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Resolved(construct),
            scope: Box::new(scope),
        };
        self.constructs.push(con)
    }

    pub fn push_unique(&mut self) -> UniqueId {
        self.uniques.push(Unique)
    }

    pub fn push_unresolved(
        &mut self,
        token: Token<'x>,
        scope: impl Scope + 'static,
    ) -> ConstructId {
        token.set_scope_of_items(self, &scope);
        if let Token::Construct(con) = token {
            con
        } else {
            let con = AnnotatedConstruct {
                definition: ConstructDefinition::Unresolved(token),
                scope: Box::new(scope),
            };
            self.constructs.push(con)
        }
    }

    pub fn get_root(&self) -> ConstructId {
        self.constructs.first().unwrap()
    }

    pub fn set_root(&mut self, def: ConstructDefinition<'x>) {
        let root = self.get_root();
        self.constructs[root].definition = def;
    }

    pub(crate) fn check(&mut self, con_id: ConstructId) {
        let con = self.get_construct_definition(con_id).dyn_clone();
        con.check(self);
    }

    pub(crate) fn is_def_equal(&mut self, left: ConstructId, right: ConstructId) -> TripleBool {
        let other = self.get_construct_definition(right).dyn_clone();
        self.get_construct_definition(left)
            .dyn_clone()
            .is_def_equal(self, &*other)
    }
}
