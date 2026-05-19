//@ run-pass
pub fn main() {
    println!("{}", std::mem::size_of_val(&1));
}
use std::prelude::*;