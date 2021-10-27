#![feature(negative_impls)]

use fake_libcore::Future;

trait Termination {}

impl<E> Termination for Result<(), E> {}
impl Termination for () {}

impl<T, F> Termination for F
where
    T: Termination,
    F: Future<Output = T> {}
