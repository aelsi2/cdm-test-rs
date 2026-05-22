//@ run-pass
#![allow(unused_mut)]
#![allow(unused_variables)]

pub fn main() {
    let mut array = [1, 2, 3];
    let pie_slice = &array[1..2];
}

use std::prelude::*;