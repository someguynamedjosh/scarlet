pub mod dependencies;
pub mod matchh;
pub mod reduce;
pub mod resolve;
pub mod substitute;
pub mod util;
mod vomit;

use crate::{
    constructs::{
        base::{
            AnnotatedConstruct, BoxedConstruct, ConstructDefinition, ConstructId, ConstructPool,
        },
        variable::{CVariable, VarType, Variable, VariablePool},
    },
    shared::Pool,
    tokens::structure::Token,
};

#[derive(Debug)]
pub struct Environment<'x> {
    pub(crate) constructs: ConstructPool<'x>,
    pub(crate) variables: VariablePool,
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

    pub fn push_unresolved(
        &mut self,
        token: Token<'x>,
        parent_scope: Option<ConstructId>,
    ) -> ConstructId {
        if token == "Theorem".into() {
            println!("{:#?}", self);
            println!("HERE");
        }
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

    pub fn push_variable(&mut self, typee: VarType, capturing: bool) -> ConstructId {
        let id = self.variables.push(Variable);
        let def = CVariable {
            capturing,
            id,
            typee,
        };
        self.push_construct(Box::new(def))
    }

    pub(crate) fn check(&mut self, con_id: ConstructId) {
        let con = self.get_construct(con_id).dyn_clone();
        con.check(self);
    }
}
