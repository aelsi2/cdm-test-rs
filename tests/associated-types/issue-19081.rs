//@ check-pass
pub trait Hasher {
    type State;

    fn hash<T: Hash<
        <Self as Hasher>::State
    >>(&self, value: &T) -> u64;
}

pub trait Hash<S> {
    fn hash(&self, state: &mut S);
}

pub fn main() {}

use std::prelude::*;