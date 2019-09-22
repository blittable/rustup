#![feature(generic_associated_types)]
//~ WARN `generic_associated_types` is incomplete

// Checking the interaction with this other feature
#![feature(associated_type_defaults)]

use std::fmt::{Debug, Display};

trait Foo {
    type Assoc;
    type Assoc2<T>;
    type Assoc3<T>;
    type WithDefault<T> = dyn Iterator<Item = T>;
    type NoGenerics;
}

struct Bar;

impl Foo for Bar {
    type Assoc = usize;
    type Assoc2<T> = Vec<T>;
    type Assoc3<T> = Vec<T>;
    type WithDefault<'a, T> = &'a dyn Iterator<T>;
    type NoGenerics = ::std::cell::Cell<i32>;
}

fn main() {}
