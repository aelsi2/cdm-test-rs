//@ run-pass

fn foo(_: *const ()) {}

pub fn main() {
    let a = 3;
    foo(&a as *const _ as *const ());
}
use std::prelude::*;