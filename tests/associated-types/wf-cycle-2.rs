//@ check-pass

trait IntoIt {
    type Item;
}

impl<I> IntoIt for I {
    type Item = ();
}

trait BaseGraph
where
    <Self::VertexIter as IntoIt>::Item: Sized,
{
    type VertexIter: IntoIt;
}

pub fn main() {}

use std::prelude::*;