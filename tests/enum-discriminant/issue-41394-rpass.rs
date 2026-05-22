//@ run-pass
//@ aux-build:issue-41394.rs

mod lib {
#[repr(u32)]
pub enum Foo {
    Foo = Private::Variant as u32
}

#[repr(u8)]
enum Private {
    Variant = 42
}

#[inline(always)]
pub fn foo() -> Foo {
    Foo::Foo
}
}

pub fn main() {
    assert_eq!(lib::foo() as u32, 42);
}

use std::prelude::*;
