use super::structure::{Environment, Value, ValueId};

impl Environment {
    fn reduce_from_scratch(&mut self, of: ValueId) -> ValueId {
        match &self.values[of] {
            Value::Any(..) => of,
            Value::BuiltinOperation(_) => todo!(),
            Value::BuiltinValue(..) => of,
            Value::From { base, variable } => {
                let (base, variable) = (*base, *variable);
                let base = self.reduce(base);
                let value = Value::From { base, variable };
                self.gpv(value)
            }
            Value::Substituting {
                base,
                target,
                value,
            } => {
                let (base, target, value) = (*base, *target, *value);
                let rbase = self.reduce(base);
                let rvalue = self.reduce(value);
                self.substitute(rbase, target, rvalue)
            }
            Value::Variant(_) => todo!(),
        }
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
