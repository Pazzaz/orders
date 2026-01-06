use crate::{
    OrderRef,
    chain::{Chain, ChainIRef},
    unique_and_bounded,
};

#[derive(Debug, Clone, Copy)]
pub struct ChainRef<'a> {
    pub(crate) order: &'a [usize],
}

impl<'a> ChainRef<'a> {
    /// Create a new `StrictRef` from a permutation of `0..s.len()`.
    pub fn new(v: &'a [usize]) -> Self {
        assert!(unique_and_bounded(v.len(), v));
        ChainRef { order: v }
    }

    pub unsafe fn new_unchecked(v: &'a [usize]) -> Self {
        ChainRef { order: v }
    }

    pub fn elements(&self) -> usize {
        self.order.len()
    }

    pub fn top(&self, n: usize) -> &[usize] {
        &self.order[..n]
    }

    pub fn to_incomplete(self) -> ChainIRef<'a> {
        let Self { order } = self;
        let elements = order.len();
        ChainIRef { elements, order }
    }
}

impl OrderRef for ChainRef<'_> {
    type Owned = Chain;

    fn to_owned(self) -> Self::Owned {
        Chain { order: self.order.to_vec() }
    }
}
