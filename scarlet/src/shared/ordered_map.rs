use std::{
    fmt::{self, Debug},
    iter::FromIterator,
};

pub type OrderedSet<K> = OrderedMap<K, ()>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct OrderedMap<K, V> {
    entries: Vec<(K, V)>,
}

impl<K, V> OrderedMap<K, V> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl<K: PartialEq + Eq + Debug, V> OrderedMap<K, V> {
    #[inline]
    fn insert_impl(&mut self, key: K, value: V, allow_replacement: bool) {
        if let Some(existing_idx) = self.entries.iter().position(|i| i.0 == key) {
            if !allow_replacement {
                panic!("Tried value insert without replacement, but a value at key {:?} already exists.", key)
            }
            self.entries[existing_idx].1 = value;
        } else {
            self.entries.push((key, value))
        }
    }

    pub fn insert_or_replace(&mut self, key: K, value: V) {
        self.insert_impl(key, value, true)
    }

    #[track_caller]
    pub fn insert_no_replace(&mut self, key: K, value: V) {
        self.insert_impl(key, value, false)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        for (candidate, value) in self {
            if candidate == key {
                return Some(value);
            }
        }
        None
    }

    pub fn contains_key(&self, key: &K) -> bool {
        for (candidate, _) in self {
            if candidate == key {
                return true;
            }
        }
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.into_iter()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn union(mut self, other: Self) -> Self {
        for (key, value) in other {
            self.insert_or_replace(key, value);
        }
        self
    }

    pub fn difference(self, keys_to_remove: &Self) -> Self {
        let mut result = Self::new();
        for (key, value) in self {
            if !keys_to_remove.contains_key(&key) {
                result.insert_no_replace(key, value)
            }
        }
        result
    }
}

impl<K: Debug, V: Debug> Debug for OrderedMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\n")?;
        for (key, value) in self {
            write!(f, "    {:?} -> {:?}\n", key, value)?;
        }
        write!(f, "}}")
    }
}

impl<K, V> Default for OrderedMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> IntoIterator for OrderedMap<K, V> {
    type IntoIter = std::vec::IntoIter<Self::Item>;
    type Item = (K, V);

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a OrderedMap<K, V> {
    type IntoIter = std::slice::Iter<'a, (K, V)>;
    type Item = &'a (K, V);

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

impl<K, V> FromIterator<(K, V)> for OrderedMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self {
            entries: iter.into_iter().collect(),
        }
    }
}