use std::{
    collections::HashMap,
    fmt::{self, Debug},
};

use super::{Value, ValueId, Variable, Variant};
use crate::shared::Pool;

#[derive(Clone, Debug)]
pub enum PathComponent {
    Member(String),
}

pub type Path = Vec<PathComponent>;

#[derive(Clone)]
pub struct Paths {
    data: HashMap<ValueId, Vec<Path>>,
}

impl Debug for Paths {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, paths) in &self.data {
            for path in paths {
                write!(f, "\n{:?} at {:?}", key, path)?;
            }
        }
        Ok(())
    }
}

impl Paths {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, value: ValueId, is_at_path: Path) {
        self.data
            .entry(value)
            .or_insert(Vec::new())
            .push(is_at_path)
    }

    pub fn get(&self, value: ValueId) -> &[Path] {
        self.data.get(&value).map(|e| &e[..]).unwrap_or(&[])
    }
}

#[derive(Clone, Debug)]
pub struct Environment {
    pub values: Pool<Value, 'L'>,
    pub variables: Pool<Variable, 'V'>,
    pub variants: Pool<Variant, 'T'>,
    pub type_cache: HashMap<ValueId, ValueId>,
    pub reduce_cache: HashMap<ValueId, ValueId>,
    pub paths: Paths,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: Pool::new(),
            variables: Pool::new(),
            variants: Pool::new(),
            type_cache: HashMap::new(),
            reduce_cache: HashMap::new(),
            paths: Paths::new(),
        }
    }
}
