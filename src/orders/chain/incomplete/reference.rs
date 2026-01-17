use crate::{
    OrderRef,
    chain::{ChainI, ChainRef},
    tied::TiedIRef,
    unique_and_bounded,
};

/// Reference to a [`ChainI`]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ChainIRef<'a> {
    pub(crate) elements: usize,
    pub(crate) order: &'a [usize],
}

impl<'a> ChainIRef<'a> {
    /// Create a reference to a strictly ordered (possible incomplete) order.
    ///
    /// # Panics
    ///
    /// Elements in `order` have to be less than `elements`, without duplicates;
    /// otherwise it panics.
    pub fn new(elements: usize, order: &'a [usize]) -> Self {
        Self::try_new(elements, order).unwrap()
    }

    /// Tries to create a reference to a strictly ordered (possible incomplete)
    /// order.
    ///
    /// Elements in `order` have to be less than `elements`, without duplicates;
    /// otherwise it returns None.
    pub fn try_new(elements: usize, order: &'a [usize]) -> Option<Self> {
        if unique_and_bounded(elements, order) { Some(ChainIRef { elements, order }) } else { None }
    }

    /// Create a reference to a strictly ordered (possible incomplete) order.
    ///
    /// # Safety
    ///
    /// Elements in `order` have to be less than `elements`, without duplicates.
    pub unsafe fn new_unchecked(elements: usize, order: &'a [usize]) -> Self {
        ChainIRef { elements, order }
    }

    pub fn order(&self) -> &[usize] {
        self.order
    }

    pub fn len(&self) -> usize {
        self.order.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn top(&self, n: usize) -> Self {
        ChainIRef::new(self.elements, &self.order[0..n])
    }

    pub fn winner(&self) -> usize {
        debug_assert!(!self.order.is_empty());
        self.order[0]
    }

    pub fn to_tied(self, tied: &'a [bool]) -> TiedIRef<'a> {
        TiedIRef::new(self.elements, self.order, tied)
    }
}

impl OrderRef for ChainIRef<'_> {
    type Owned = ChainI;

    fn to_owned(self) -> Self::Owned {
        ChainI::new(self.elements, self.order.to_vec())
    }
}

impl<'a> TryFrom<ChainIRef<'a>> for ChainRef<'a> {
    type Error = ();

    /// Convert to complete order, returns `Err(())` if the order isn't actually
    /// complete.
    fn try_from(ChainIRef { elements, order }: ChainIRef<'a>) -> Result<Self, Self::Error> {
        if elements == order.len() { Ok(ChainRef { order }) } else { Err(()) }
    }
}
