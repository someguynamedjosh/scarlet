use super::structure::{Environment, ValueId};

impl Environment {
    fn reduce_from_scratch(&mut self, of: ValueId) -> ValueId {
        // TODO: Not this.
        of
    }

    pub fn reduce(&mut self, of: ValueId) -> ValueId {
        if let Some(cached) = self.reduce_cache.get(&of) {
            *cached
        } else {
            let reduced = self.reduce_from_scratch(of);
            self.reduce_cache.insert(of, reduced);
            self.get_type(reduced);
            reduced
        }
    }
}
