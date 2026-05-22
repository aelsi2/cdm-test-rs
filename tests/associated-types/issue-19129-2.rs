//@ check-pass
#![allow(unused_variables)]

trait Trait<Input> {
    type Output;

    fn method(&self, i: Input) -> bool { false }
}

pub fn main() {}

use std::prelude::*;