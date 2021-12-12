use super::{ConstructId, Environment};
use crate::constructs::{base::BoxedConstruct, AnnotatedConstruct};

impl<'x> Environment<'x> {
    pub fn get_construct(&self, con_id: ConstructId) -> &AnnotatedConstruct {
        &self.constructs[con_id]
    }

    pub fn get_construct_definition(&mut self, con_id: ConstructId) -> &BoxedConstruct {
        let con_id = self.resolve(con_id);
        println!("{:#?}\n{:?}", self, con_id);
        self.constructs[con_id].definition.as_resolved().unwrap()
    }
}
