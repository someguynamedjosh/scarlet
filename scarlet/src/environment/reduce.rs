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
        if let ConstructDefinition::Resolved(con) = &self.constructs[con_id].definition {
            let con = con.dyn_clone();
            let reduced = con.reduce(self);
            self.constructs[con_id].definition = reduced;
        }
    }
}
