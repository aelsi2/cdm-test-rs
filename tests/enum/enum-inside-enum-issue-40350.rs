//@ check-pass

enum E {
    A = {
        enum F { B }
        0
    }
}

pub fn main() {}

use std::prelude::*;