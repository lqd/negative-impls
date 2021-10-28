#![feature(negative_impls)]

pub trait Future {}

// impl<E> !Future for Option<E> {} // version A: breaks fake_libstd
// impl !Future for Option<()> {} // version B: works
impl<E: ?Sized> !Future for Option<E> {} // version C: works
