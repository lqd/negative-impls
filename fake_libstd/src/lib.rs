#![feature(negative_impls)]

use fake_libcore::Future;

trait Termination {}

impl<E> Termination for Option<E> {}

impl<F> Termination for F where F: Future {}
