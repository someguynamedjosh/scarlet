use super::{ConstructId, Environment};

impl<'x> Environment<'x> {
    pub fn reduce(&mut self, con_id: ConstructId) -> ConstructId {
        let result = self.reduce_impl(con_id);
        // println!("{:#?}", self);
        debug_assert_eq!(
            result,
            self.reduce_impl(result),
            "{:?} reduces to {:?}, but that reduces to another value!",
            con_id,
            result
        );
        result
    }

    fn reduce_impl(&mut self, con_id: ConstructId) -> ConstructId {
        let con_id = self.resolve(con_id);
        let con = self.constructs[con_id]
            .definition
            .as_resolved()
            .unwrap()
            .dyn_clone();
        con.check(self);
        con.reduce(self, con_id)
    }
}
