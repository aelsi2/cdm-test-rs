//@ run-pass

pub fn main() {
    let _x: &mut [isize] = &mut [ 1, 2, 3 ];
}

use std::prelude::*;