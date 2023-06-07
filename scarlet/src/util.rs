use std::{
    cell::RefCell,
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{BuildHasher, Hash, Hasher},
    rc::Rc,
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
        if cfg!(debug_assertions) {
            key.assertions();
        }
        let hash = {
            let mut hasher = self.hasher().build_hasher();
            key.hash(&mut hasher);
            hasher.finish()
        };
        self.raw_entry().from_hash(hash, |other| key.equals(other))
    }
}

pub fn rcrc<T>(value: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(value))
}

#[macro_export]
macro_rules! impl_any_eq_from_regular_eq {
    ($ConstructName:ident) => {
        impl crate::shared::AnyEq for $ConstructName {
            fn eq(&self, other: &dyn crate::shared::AnyEq) -> bool {
                (other as &dyn std::any::Any)
                    .downcast_ref::<Self>()
                    .map(|x| self == x)
                    .unwrap_or(false)
            }
        }
    };
}

/// These are just handy aliases to existing functions.
pub trait PtrExtension {
    fn is_same_instance_as(&self, other: &Self) -> bool;
    /// Returns a new pointer to the same object that self points to.
    fn ptr_clone(&self) -> Self;
}

impl<T> PtrExtension for Rc<T> {
    fn is_same_instance_as(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }

    fn ptr_clone(&self) -> Self {
        Rc::clone(self)
    }
}
