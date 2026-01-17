use crate::{
    OrderRef,
    chain::{Chain, ChainIRef},
    unique_and_bounded,
};

/// Reference to a [`Chain`]
#[derive(Debug, Clone, Copy)]
pub struct ChainRef<'a> {
    pub(crate) order: &'a [usize],
}

impl<'a> ChainRef<'a> {
    /// Create a new `ChainRef` from a permutation.
    ///
    /// # Panics
    ///
    /// Panics if `v` is not a valid permutation.
    pub fn new(v: &'a [usize]) -> Self {
        Self::try_new(v).expect("slice should be a permutation")
    }

    /// Create a new `ChainRef` from a permutation.
    ///
    /// Returns [`None`] if `v` is not a valid permutation.
    pub fn try_new(v: &'a [usize]) -> Option<Self> {
        if unique_and_bounded(v.len(), v) { Some(ChainRef { order: v }) } else { None }
    }

    /// Create a new `ChainRef` from a permutation.
    ///
    /// # Safety
    ///
    /// Assumes `v` is not a valid permutation.
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
