//@ run-pass
enum Void {}
fn foo(_: Result<(Void, u32), (Void, String)>) {}
pub fn main() {
    let _: fn(_) = foo;
}

use std::prelude::*;