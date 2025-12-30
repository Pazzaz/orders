//! # Total orders, allowing ties

mod dense;
mod dense_complete;
mod groups;
mod split_ref;
mod tied_incomplete;
mod tied_incomplete_ref;

pub use dense::*;
pub use dense_complete::*;
pub use groups::*;
use rand::{Rng, distr::Bernoulli, seq::SliceRandom};
use split_ref::SplitRef;
pub use tied_incomplete::*;
pub use tied_incomplete_ref::*;

use crate::{
    Order, OrderOwned, OrderRef, cardinal::CardinalRef, partial_order::PartialOrderManual,
    unique_and_bounded,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Tied {
    order: Vec<usize>,
    tied: Vec<bool>,
}

impl Clone for Tied {
    fn clone(&self) -> Self {
        Self { order: self.order.clone(), tied: self.tied.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.order.clone_from(&source.order);
        self.tied.clone_from(&source.tied);
    }
}

impl Tied {
    pub fn new(order: Vec<usize>, tied: Vec<bool>) -> Self {
        Self::try_new(order, tied).unwrap()
    }

    pub fn try_new(order: Vec<usize>, tied: Vec<bool>) -> Option<Self> {
        let correct_len = order.is_empty() && tied.is_empty() || tied.len() + 1 == order.len();
        if correct_len && unique_and_bounded(order.len(), &order) {
            Some(Tied { order, tied })
        } else {
            None
        }
    }

    pub unsafe fn new_unchecked(order: Vec<usize>, tied: Vec<bool>) -> Self {
        Tied { order, tied }
    }

    pub fn order(&self) -> &[usize] {
        &self.order
    }

    pub fn tied(&self) -> &[bool] {
        &self.tied
    }

    /// Clones from `source` to `self`, similar to [`Clone::clone_from`].
    pub fn clone_from_ref(&mut self, source: TiedRef) {
        self.order.clone_from_slice(source.order());
        self.tied.clone_from_slice(source.tied());
    }

    /// Create a new ranking of `elements`, where every element is tied.
    ///
    /// ```
    /// use orders::{OrderOwned, tied::Tied};
    ///
    /// let c = 10;
    /// let rank = Tied::new_tied(c);
    /// assert_eq!(rank.as_ref().winners().len(), c);
    /// ```
    pub fn new_tied(elements: usize) -> Self {
        if elements == 0 {
            return Tied::new(Vec::new(), Vec::new());
        }
        let mut order = Vec::with_capacity(elements);
        for i in 0..elements {
            order.push(i);
        }
        let tied = vec![true; elements - 1];
        Tied::new(order, tied)
    }

    /// Generate a random tied ranking of `elements`.
    pub fn random<R: Rng>(rng: &mut R, elements: usize) -> Self {
        if elements == 0 {
            return Tied::new(Vec::new(), Vec::new());
        }
        let mut order: Vec<usize> = (0..elements).collect();
        order.shuffle(rng);
        let tied_len = elements - 1;
        let mut tied = Vec::with_capacity(tied_len);
        let d = Bernoulli::new(0.5).unwrap();
        for _ in 0..tied_len {
            tied.push(rng.sample(d));
        }
        Tied::new(order, tied)
    }
}

impl<'a> From<CardinalRef<'a>> for Tied {
    fn from(value: CardinalRef) -> Self {
        let mut list: Vec<(usize, usize)> = value.values().iter().copied().enumerate().collect();
        list.sort_by(|(_, a), (_, b)| a.cmp(b).reverse());
        let tied: Vec<bool> = list.windows(2).map(|w| w[0].1 == w[1].1).collect();
        let order: Vec<usize> = list.into_iter().map(|(i, _)| i).collect();
        Tied::new(order, tied)
    }
}

impl Order for Tied {
    fn elements(&self) -> usize {
        self.order.len()
    }

    fn len(&self) -> usize {
        self.order.len()
    }

    fn to_partial(self) -> crate::partial_order::PartialOrder {
        let mut manual = PartialOrderManual::new(self.elements());
        let mut seen: Vec<usize> = Vec::with_capacity(self.len());
        for group in self.as_ref().iter_groups() {
            for i in group {
                for j in &seen {
                    manual.set(*i, *j);
                }
            }
            seen.extend_from_slice(group);
        }

        // SAFETY: Each element has no relation to any element in their tied group, but
        // is smaller than every element seen before.
        unsafe { manual.finish_unchecked() }
    }
}

impl<'a> OrderOwned<'a> for Tied {
    type Ref = TiedRef<'a>;

    fn as_ref(&'a self) -> Self::Ref {
        TiedRef::new(&self.order, &self.tied)
    }
}

impl From<Tied> for TiedI {
    fn from(Tied { order, tied }: Tied) -> Self {
        TiedI::new(order.len(), order, tied)
    }
}

pub struct TiedRef<'a> {
    order_tied: SplitRef<'a>,
}

impl<'a> TiedRef<'a> {
    pub fn new(order: &'a [usize], tied: &'a [bool]) -> Self {
        Self::try_new(order, tied).unwrap()
    }

    pub fn try_new(order: &'a [usize], tied: &'a [bool]) -> Option<Self> {
        let correct_len = order.is_empty() && tied.is_empty() || tied.len() + 1 == order.len();
        if correct_len && unique_and_bounded(order.len(), order) {
            Some(TiedRef { order_tied: SplitRef::new(order, tied) })
        } else {
            None
        }
    }

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
        let ti: TiedIRef = self.into();
        ti.winners()
    }

    pub fn iter_groups(&self) -> GroupIterator<'_> {
        GroupIterator { order: self.into() }
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

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};

    use super::*;
    use crate::{partial_order, tests::std_rng};

    fn valid(td: &Tied) -> bool {
        if td.order.len().saturating_sub(1) != td.tied.len() {
            return false;
        }

        // Each element is ordered once
        let mut seen = vec![false; td.len()];
        for i in &td.order {
            match seen.get_mut(*i) {
                Some(v) => {
                    if *v {
                        return false;
                    } else {
                        *v = true;
                    }
                }
                None => return false,
            }
        }

        true
    }

    impl Arbitrary for Tied {
        fn arbitrary(g: &mut Gen) -> Self {
            let mut elements: usize = Arbitrary::arbitrary(g);

            // `Arbitrary` for numbers will generate "problematic" examples such as
            // `usize::max_value()` and `usize::min_value()` but we'll use them to
            // allocate vectors so we'll limit them.
            elements = elements % g.size();

            Tied::random(&mut std_rng(g), elements)
        }
    }

    #[quickcheck]
    fn generate(orders: Tied) -> bool {
        valid(&orders)
    }

    #[quickcheck]
    fn partial(orders: Tied) -> bool {
        partial_order::tests::valid(&orders.to_partial())
    }
}
