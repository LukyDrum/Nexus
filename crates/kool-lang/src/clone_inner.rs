use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

pub trait CloneInner {
    type Inner;

    fn clone_inner(&self) -> Self::Inner;
}

impl<T: Clone> CloneInner for Rc<T> {
    type Inner = T;

    fn clone_inner(&self) -> Self::Inner {
        (**self).clone()
    }
}

impl<T: Clone> CloneInner for Arc<T> {
    type Inner = T;

    fn clone_inner(&self) -> Self::Inner {
        (**self).clone()
    }
}

impl<T: Clone> CloneInner for RwLock<T> {
    type Inner = T;

    fn clone_inner(&self) -> Self::Inner {
        self.read().expect("Lock poisoned").clone()
    }
}
