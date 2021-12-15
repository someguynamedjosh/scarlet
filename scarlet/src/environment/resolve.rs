use itertools::Itertools;

use super::{ConstructDefinition, ConstructId, Environment};
use crate::{
    constructs::{
        as_variable,
        substitution::{CSubstitution, Substitutions},
        variable::CVariable, self,
    },
    scope::{SPlain, Scope},
    tokens::structure::Token,
    transform::{self, ApplyContext},
};

impl<'x> Environment<'x> {
    pub fn resolve_all(&mut self) {
        let mut next_id = self.constructs.first();
        while let Some(id) = next_id {
            self.resolve(id);
            next_id = self.constructs.next(id);
        }
    }

    pub fn resolve(&mut self, con_id: ConstructId) -> ConstructId {
        todo!()
    }
}
