use rand::{
    Rng,
    seq::{IteratorRandom, SliceRandom},
};

use crate::{
    Order, OrderOwned,
    chain::{Chain, ChainIRef},
    partial_order::{PartialOrder, PartialOrderManual},
    unique_and_bounded,
};

/// Incomplete version of [`Chain`]
#[derive(Debug, PartialEq, Eq)]
pub struct ChainI {
    pub(crate) elements: usize,
    pub(crate) order: Vec<usize>,
}

impl Clone for ChainI {
    fn clone(&self) -> Self {
        Self { elements: self.elements, order: self.order.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.elements = source.elements;
        self.order.clone_from(&source.order);
    }
}

impl ChainI {
    pub fn new(elements: usize, order: Vec<usize>) -> Self {
        Self::try_new(elements, order).unwrap()
    }

    pub fn try_new(elements: usize, order: Vec<usize>) -> Option<Self> {
        if unique_and_bounded(elements, &order) { Some(ChainI { elements, order }) } else { None }
    }

    pub unsafe fn new_unchecked(elements: usize, order: Vec<usize>) -> Self {
        ChainI { elements, order }
    }

    /// Clones from `source` to `self`, similar to [`Clone::clone_from`].
    pub fn clone_from_ref(&mut self, source: ChainIRef) {
        self.order.clone_from_slice(source.order);
        self.elements = source.elements;
    }

    pub fn random<R: Rng>(rng: &mut R, elements: usize) -> ChainI {
        if elements == 0 {
            ChainI { order: Vec::new(), elements }
        } else {
            let len = rng.random_range(0..elements);

            let mut order = (0..elements).choose_multiple(rng, len);
            order.shuffle(rng);
            ChainI { order, elements }
        }
    }
}

impl TryFrom<ChainI> for Chain {
    type Error = ();

    /// Convert to total order. Returns `Err` if not all elements are ranked.
    fn try_from(ChainI { elements, order }: ChainI) -> Result<Self, Self::Error> {
        if elements == order.len() { Ok(Chain { order }) } else { Err(()) }
    }
}

impl Order for ChainI {
    fn elements(&self) -> usize {
        self.elements
    }

    fn len(&self) -> usize {
        self.order.len()
    }

    fn to_partial(self) -> PartialOrder {
        let mut manual = PartialOrderManual::new(self.elements());
        for (i1, e1) in self.order.iter().enumerate() {
            for e2 in &self.order[(i1 + 1)..] {
                manual.set(*e2, *e1);
            }
        }
        // SAFETY: We set the relations in `self.order`, including transitive relations.
        // The elements in `rest` have no relations with eachother, or the
        // non-ordered elements.
        unsafe { manual.finish_unchecked() }
    }
}

impl<'a> OrderOwned<'a> for ChainI {
    type Ref = ChainIRef<'a>;

    fn as_ref(&'a self) -> Self::Ref {
        ChainIRef { elements: self.elements, order: &self.order }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};

    use super::*;
    use crate::{
        partial_order::tests::valid,
        tests::{BoundedArbitrary, std_rng},
    };

    impl Arbitrary for ChainI {
        fn arbitrary(g: &mut Gen) -> Self {
            let elements: usize = BoundedArbitrary::arbitrary(g);
            ChainI::random(&mut std_rng(g), elements)
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            let x = self.clone();
            let iter = (0..(x.len().saturating_sub(1)))
                .rev()
                .map(move |i| ChainI::new(x.elements, x.order[0..i].to_vec()));
            Box::new(iter)
        }
    }

    #[quickcheck]
    fn as_partial(b: ChainI) -> bool {
        let po = b.to_partial();
        valid(&po)
    }

    #[quickcheck]
    fn as_partial_correct(b: ChainI) -> bool {
        let po = b.clone().to_partial();
        if !valid(&po) {
            return false;
        }
        for (i, vi) in b.order.iter().enumerate() {
            for (j, vj) in b.order.iter().enumerate() {
                let index_cmp = j.cmp(&i);
                if let Some(value_cmp) = po.ord(*vi, *vj) {
                    if index_cmp != value_cmp {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        let mut values = b.order;
        values.sort();
        let rest: Vec<usize> =
            (0..b.elements).filter(|x| !values.binary_search(x).is_ok()).collect();
        for &p in &values {
            for &q in &rest {
                if po.le(q, p) || po.le(p, q) {
                    return false;
                }
            }
        }
        for &r1 in &rest {
            for &r2 in &rest {
                if r1 == r2 {
                    if !po.eq(r1, r2) {
                        return false;
                    }
                } else if po.le(r1, r2) {
                    return false;
                }
            }
        }
        valid(&po)
    }

    #[quickcheck]
    fn len(b: ChainI) -> bool {
        b.len() <= b.elements()
    }
}
