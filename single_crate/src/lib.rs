#![feature(negative_impls)]

trait Termination {}
trait Future {
    type Output;
}

impl<E> Termination for Result<(), E> {}
impl Termination for () {}

impl !Future for () {}
impl<E> !Future for Result<(), E> {}

impl<T, F> Termination for F
where
    T: Termination,
    F: Future<Output = T> {}
