pub enum OwnedOrBorrowed<'a, T: ?Sized> {
    Owned(Box<T>),
    Borrowed(&'a T),
}

impl<'a, T: ?Sized> AsRef<T> for OwnedOrBorrowed<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            OwnedOrBorrowed::Owned(data) => &*data,
            OwnedOrBorrowed::Borrowed(data) => *data,
        }
    }
}
