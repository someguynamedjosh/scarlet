use super::{ConstructDefinition, ConstructId, Environment};
use crate::{
    constructs::base::{BoxedConstruct, Construct},
    tokens::structure::Token,
};

impl<'x> Environment<'x> {
    pub fn get_construct(&mut self, con_id: ConstructId) -> &BoxedConstruct {
        let con_id = self.resolve(con_id);
        self.constructs[con_id].definition.as_resolved().unwrap()
    }
}
