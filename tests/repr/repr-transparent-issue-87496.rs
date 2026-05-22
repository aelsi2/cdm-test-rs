// Regression test for the ICE described in #87496.

//@ check-pass

#[repr(transparent)]
struct TransparentCustomZst(());
unsafe extern "C" {
    fn good17(p: TransparentCustomZst);
}

pub fn main() {}

use std::prelude::*;
