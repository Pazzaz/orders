use crate::{
    OrderRef,
    specific::Specific,
    tied::{GroupIterator, Tied, TiedIRef, split_ref::SplitRef},
    unique_and_bounded,
};

pub struct TiedRef<'a> {
    order_tied: SplitRef<'a>,
}

impl<'a> TiedRef<'a> {
    /// Create a new `TiedRef` from a permutation and a list denoting ties.
    ///
    /// # Panics
    ///
    /// Panics if `order` is not a valid permutation or `tied` is the wrong
    /// length.
    pub fn new(order: &'a [usize], tied: &'a [bool]) -> Self {
        Self::try_new(order, tied).unwrap()
    }

    /// Create a new `TiedRef` from a permutation and a list denoting ties.
    ///
    /// Returns `None` if `order` is not a valid permutation or `tied` is the
    /// wrong length.
    pub fn try_new(order: &'a [usize], tied: &'a [bool]) -> Option<Self> {
        let correct_len = order.is_empty() && tied.is_empty() || tied.len() + 1 == order.len();
        if correct_len && unique_and_bounded(order.len(), order) {
            Some(TiedRef { order_tied: SplitRef::new(order, tied) })
        } else {
            None
        }
    }

    /// Create a new `TiedRef` from a permutation and a list denoting ties.
    ///
    /// # Safety
    ///
    /// Assumes `order` is a valid permutation and `tied` is the correct length.
    pub unsafe fn new_unchecked(order: &'a [usize], tied: &'a [bool]) -> Self {
        TiedRef { order_tied: SplitRef::new(order, tied) }
    }

    pub fn elements(&self) -> usize {
        self.order().len()
    }

    pub fn order(&self) -> &'a [usize] {
        self.order_tied.a()
    }

    pub fn tied(&self) -> &'a [bool] {
        self.order_tied.b()
    }

    pub fn winners(&self) -> &'a [usize] {
        TiedIRef::from(self).winners()
    }

    pub fn winner<R: rand::Rng>(&self, rng: &mut R) -> Specific {
        TiedIRef::from(self).winner(rng)
    }

    pub fn iter_groups(&self) -> GroupIterator<'_> {
        TiedIRef::from(self).iter_groups()
    }
}

impl<'a> OrderRef for TiedRef<'a> {
    type Owned = Tied;

    fn to_owned(self) -> Self::Owned {
        Tied::new(self.order().to_vec(), self.tied().to_vec())
    }
}

impl<'a> From<TiedRef<'a>> for TiedIRef<'a> {
    fn from(value: TiedRef<'a>) -> Self {
        TiedIRef::new(value.elements(), value.order(), value.tied())
    }
}

impl<'a> From<&TiedRef<'a>> for TiedIRef<'a> {
    fn from(value: &TiedRef<'a>) -> Self {
        TiedIRef::new(value.elements(), value.order(), value.tied())
    }
}
