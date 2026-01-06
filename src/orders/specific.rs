use rand::Rng;

use crate::{Order, OrderOwned, partial_order::PartialOrderManual};

#[derive(Debug, Clone, Copy)]
pub struct Specific {
    pub(crate) value: usize,
    elements: usize,
}

impl Specific {
    pub fn new(value: usize, elements: usize) -> Self {
        assert!(value < elements);
        Self { value, elements }
    }

    pub fn random<R: Rng>(rng: &mut R, elements: usize) -> Self {
        assert_ne!(elements, 0);
        let value = rng.random_range(0..elements);
        Self { value, elements }
    }
}

impl Order for Specific {
    fn elements(&self) -> usize {
        self.elements
    }

    fn len(&self) -> usize {
        1
    }

    fn to_partial(self) -> crate::partial_order::PartialOrder {
        let mut tmp = PartialOrderManual::new(self.elements());
        // For every element...
        for i in 0..self.elements {
            // ...except the chosen value...
            if i == self.value {
                continue;
            }
            // ...they are all less than the chosen value
            tmp.set(i, self.value);
        }
        // SAFETY: There won't be any transitive relations between elements, and we
        // iterated through every pair of elements, so we've set every
        // relation.
        unsafe { tmp.finish_unchecked() }
    }
}

impl<'a> OrderOwned<'a> for Specific {
    type Ref = &'a Specific;

    fn as_ref(&'a self) -> Self::Ref {
        &self
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};

    use super::*;
    use crate::{partial_order::tests::valid, tests::std_rng};

    impl Arbitrary for Specific {
        fn arbitrary(g: &mut Gen) -> Self {
            // Generate a random number of elements
            let v: usize = <usize as Arbitrary>::arbitrary(g);

            // Modulo it by `size` to avoid problematic values
            // and add 1 ot avoid 0
            let elements = (v % g.size()).saturating_add(1);

            // Then randomly choose the specific value
            Specific::random(&mut std_rng(g), elements)
        }
    }

    #[quickcheck]
    fn as_partial(b: Specific) -> bool {
        let po = b.to_partial();
        valid(&po)
    }
}
