use crate::stage2::structure::{ConstructId, Environment};

impl<'x> Environment<'x> {
    pub fn reduce(&mut self, item: ConstructId<'x>) -> ConstructId<'x> {
        if let Some(reduction) = &self.items[item].cached_reduction {
            *reduction
        } else if self.query_stack_contains(item) {
            println!("{:#?}", self);
            todo!("Handle recursive reduction on {:?}", item)
        } else {
            let result = self.with_query_stack_frame(item, |this| this.reduce_from_scratch(item));
            self.items[item].cached_reduction = Some(result);
            self.get_deps(item);
            self.get_deps(result);
            // println!("{:#?}", self);
            // println!("{:?} becomes {:?}", item, result);
            assert!(self.get_deps(result).len() <= self.get_deps(item).len());
            // println!("{:#?}", self);
            assert_eq!(self.reduce(result), result);
            result
        }
    }

    fn reduce_from_scratch(&mut self, item: ConstructId<'x>) -> ConstructId<'x> {
        let definition = self.items[item].definition.as_ref().unwrap();
        definition.reduce(item, self)
    }
}
