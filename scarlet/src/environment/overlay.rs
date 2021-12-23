use super::Environment;
use crate::{constructs::ConstructId, shared::OwnedOrBorrowed};

#[derive(Debug)]
pub struct Overlay<'e, 'x, T: Default> {
    env: &'e mut Environment<'x>,
    data: Vec<T>,
}

impl<'e, 'x, T: Default> Overlay<'e, 'x, T> {
    pub fn new(env: &'e mut Environment<'x>) -> Self {
        Self {
            env,
            data: Vec::new(),
        }
    }

    pub fn env(&self) -> &Environment<'x> {
        self.env
    }

    pub fn env_mut(&mut self) -> &mut Environment<'x> {
        self.env
    }

    pub fn get(&self, id: ConstructId) -> OwnedOrBorrowed<T> {
        if id.index < self.data.len() {
            OwnedOrBorrowed::Borrowed(&self.data[id.index])
        } else {
            OwnedOrBorrowed::Owned(Default::default())
        }
    }

    pub fn get_mut(&mut self, id: ConstructId) -> &mut T {
        while id.index >= self.data.len() {
            self.data.push(Default::default());
        }
        &mut self.data[id.index]
    }

    pub fn set(&mut self, id: ConstructId, value: T) {
        *self.get_mut(id) = value;
    }
}
