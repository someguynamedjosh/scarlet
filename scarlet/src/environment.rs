mod resolve;

use crate::{
    constructs::{
        base::{
            AnnotatedConstruct, BoxedConstruct, ConstructDefinition, ConstructId, ConstructPool,
        },
        variable::{CVariable, VarType, Variable, VariablePool},
    },
    shared::{Id, Pool},
    tokens::structure::Token,
};

#[derive(Debug)]
pub struct Environment<'x> {
    constructs: ConstructPool<'x>,
    variables: VariablePool,
}

impl<'x> Environment<'x>
where
    ConstructId: 'static,
{
    pub fn new() -> Self {
        Self {
            constructs: Pool::new(),
            variables: Pool::new(),
        }
    }

    pub fn push_placeholder(&mut self) -> ConstructId {
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Placeholder,
            parent_scope: None,
        };
        self.constructs.push(con)
    }

    pub fn push_construct(&mut self, construct: BoxedConstruct) -> ConstructId {
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Resolved(construct),
            parent_scope: None,
        };
        self.constructs.push(con)
    }

    pub fn push_unresolved(&mut self, token: Token<'x>) -> ConstructId {
        let con = AnnotatedConstruct {
            definition: ConstructDefinition::Unresolved(token),
            parent_scope: None,
        };
        self.constructs.push(con)
    }

    pub fn push_variable(&mut self, typee: VarType, capturing: bool) -> ConstructId {
        let id = self.variables.push(Variable);
        let def = CVariable {
            capturing,
            id,
            typee,
        };
        self.push_construct(Box::new(def))
    }

    pub(crate) fn check(&self, con: ConstructId) {
        // todo!()
    }
}
