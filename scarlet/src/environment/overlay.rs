use super::Environment;
use crate::{item::ItemPtr, shared::OwnedOrBorrowed};

#[derive(Debug)]
pub struct Overlay<'e, 'x, T: Default> {
    env: &'e mut Environment,
    data: Vec<T>,
}

impl<'e, 'x, T: Default> Overlay<'e, 'x, T> {
    pub fn new(env: &'e mut Environment) -> Self {
        Self {
            env,
            data: Vec::new(),
        }
    }

    pub fn env(&self) -> &Environment {
        self.env
    }

    pub fn env_mut(&mut self) -> &mut Environment {
        self.env
    }

    pub fn get(&self, id: ItemPtr) -> OwnedOrBorrowed<T> {
        if id.index < self.data.len() {
            OwnedOrBorrowed::Borrowed(&self.data[id.index])
        } else {
            OwnedOrBorrowed::Owned(Default::default())
        }
    }

    pub fn get_mut(&mut self, id: ItemPtr) -> &mut T {
        while id.index >= self.data.len() {
            self.data.push(Default::default());
        }
        &mut self.data[id.index]
    }

    pub fn set(&mut self, id: ItemPtr, value: T) {
        *self.get_mut(id) = value;
    }
}
