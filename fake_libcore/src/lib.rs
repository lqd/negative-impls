#![feature(negative_impls)]

pub trait Future {
    type Output;
}

impl !Future for () {}
impl<E> !Future for Result<(), E> {}
