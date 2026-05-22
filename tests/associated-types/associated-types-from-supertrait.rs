//@ run-pass

trait Foo: Iterator<Item = i32> {}
trait Bar: Foo {}

pub fn main() {
    let _: &dyn Bar;
}

use std::prelude::*;