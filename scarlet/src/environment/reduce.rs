use std::borrow::Cow;

use super::{ConstructDefinition, ConstructId, Environment};
use crate::{
    constructs::{
        self,
        substitution::{CSubstitution, Substitutions},
        variable::CVariable,
    },
    shared::OrderedMap,
    tokens::structure::Token,
    transform::{self, ApplyContext},
};

impl<'x> Environment<'x> {
    pub fn reduce(&mut self, con_id: ConstructId) {
        let con = self.get_construct(con_id).dyn_clone();
        let reduced = con.reduce(self);
        self.constructs[con_id].definition = reduced;
    }

    pub fn reduce_all(&mut self) {
        let mut next_id = self.constructs.first();
        while let Some(id) = next_id {
            self.reduce(id);
            next_id = self.constructs.next(id);
        }
    }
}
