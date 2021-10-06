use super::structure::{Environment, Value, ValueId};

impl Environment {
    pub fn reduce_all(&mut self) {
        if let Some(start) = self.values.first() {
            let mut id = start;
            loop {
                self.reduce(id);
                match self.values.next(id) {
                    Some(next) => id = next,
                    None => break,
                }
            }
        }
    }

    fn reduce_from_scratch(&mut self, of: ValueId) -> ValueId {
        match &self.values[of].value {
            Value::Any { id, typee } => {
                let (id, typee) = (*id, *typee);
                let typee = self.reduce(typee);
                let value = Value::Any { id, typee };
                self.gpv(value)
            }
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
            Value::Variant { id, typee } => {
                let (id, typee) = (*id, *typee);
                let typee = self.reduce(typee);
                let value = Value::Variant { id, typee };
                self.gpv(value)
            }
        }
    }

    pub fn reduce(&mut self, of: ValueId) -> ValueId {
        if let Some(cached) = self.values[of].cached_reduction {
            cached
        } else {
            let reduced = self.reduce_from_scratch(of);
            self.values[of].cached_reduction = Some(reduced);
            self.values[reduced].cached_reduction = Some(reduced);
            debug_assert_eq!(self.reduce(reduced), reduced);
            let typee = self.get_type(of);
            debug_assert_eq!(typee, self.get_type(reduced));
            self.values[reduced].cached_type = Some(typee);
            reduced
        }
    }
}
