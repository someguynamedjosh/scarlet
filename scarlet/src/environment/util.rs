use std::convert::TryInto;

use super::{ConstructId, Environment};
use crate::constructs::base::BoxedConstruct;

impl<'x> Environment<'x> {
    pub fn get_construct(&mut self, con_id: ConstructId) -> &BoxedConstruct {
        let con_id = self.resolve(con_id);
        println!("{:#?}\n{:?}", self, con_id);
        self.constructs[con_id].definition.as_resolved().unwrap()
    }
}
