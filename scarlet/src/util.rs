use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{BuildHasher, Hash, Hasher, SipHasher},
};

pub trait Ignorable {
    fn ignore(&self) {}
}

impl<R, E> Ignorable for Result<R, E> {}

/// Indicates that the type it is implemented on functions equivalently to the
/// type in its template parameter.
pub trait Isomorphism<EquivalentType: Eq + Hash>: Eq + Hash + Clone + Sized {
    fn convert(self) -> EquivalentType;
    fn equals(&self, other: &EquivalentType) -> bool;

    fn assertions(&self) {
        let other = self.clone().convert();
        let hash1 = {
            let mut hasher = DefaultHasher::new();
            self.hash(&mut hasher);
            hasher.finish()
        };
        let hash2 = {
            let mut hasher = DefaultHasher::new();
            other.hash(&mut hasher);
            hasher.finish()
        };
        assert_eq!(hash1, hash2, "The 'equivalent' value has a different hash!");
        assert!(
            self.equals(&other),
            "The 'equivalent' value is not equal to the original value!"
        );
    }
}

pub trait IsomorphicKeyIndexable<
    OriginalKey: Eq + Hash,
    IsomorphicKey: Isomorphism<OriginalKey>,
    Result,
>
{
    fn iso_get(&self, key: &IsomorphicKey) -> Option<(&OriginalKey, &Result)>;
}

impl<OriginalKey: Eq + Hash, IsomorphicKey: Isomorphism<OriginalKey>, Result>
    IsomorphicKeyIndexable<OriginalKey, IsomorphicKey, Result> for HashMap<OriginalKey, Result>
{
    fn iso_get(&self, key: &IsomorphicKey) -> Option<(&OriginalKey, &Result)> {
        let hash = {
            let mut hasher = self.hasher().build_hasher();
            key.hash(&mut hasher);
            hasher.finish()
        };
        self.raw_entry().from_hash(hash, |other| key.equals(other))
    }
}
