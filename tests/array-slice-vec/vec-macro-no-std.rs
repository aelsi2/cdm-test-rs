//@ run-pass
//@ ignore-emscripten no no_std executables
//@ ignore-wasm different `main` convention

use alloc::vec::Vec;

// Issue #16806

pub fn main() {
    let x: Vec<u8> = vec![0, 1, 2];
    match x.last() {
        Some(&2) => (),
        _ => panic!(),
    }
}

use std::prelude::*;
