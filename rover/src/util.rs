use std::{cell::RefCell, rc::Rc};

pub(crate) fn indented(source: &str) -> String {
    source.replace("\n", "\n    ")
}

pub type Rcrc<T> = Rc<RefCell<T>>;

pub fn new_rcrc<T>(value: T) -> Rcrc<T> {
    Rc::new(RefCell::new(value))
}
