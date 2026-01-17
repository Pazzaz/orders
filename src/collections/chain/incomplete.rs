use rand::{
    distr::{Distribution, Uniform},
    seq::SliceRandom,
};

use crate::{
    chain::ChainIRef,
    collections::{AddError, DenseOrders, chain::ChainDense},
};

/// Packed list of [`ChainI`](crate::chain::ChainI)
#[derive(Debug)]
pub struct ChainIDense {
    pub(crate) orders: Vec<usize>,

    // End position of order
    pub(crate) order_end: Vec<usize>,
    pub(crate) elements: usize,
}

impl Clone for ChainIDense {
    fn clone(&self) -> Self {
        Self {
            orders: self.orders.clone(),
            order_end: self.order_end.clone(),
            elements: self.elements,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.orders.clone_from(&source.orders);
        self.order_end.clone_from(&source.order_end);
        self.elements = source.elements;
    }
}

impl ChainIDense {
    pub fn new(elements: usize) -> Self {
        ChainIDense { orders: Vec::new(), order_end: Vec::new(), elements }
    }

    pub fn elements(&self) -> usize {
        self.elements
    }

    pub fn iter(&self) -> impl Iterator<Item = ChainIRef<'_>> {
        (0..self.len()).map(|i| self.get(i))
    }
}

impl<'a> DenseOrders<'a> for ChainIDense {
    type Order = ChainIRef<'a>;

    fn elements(&self) -> usize {
        self.elements
    }

    fn len(&self) -> usize {
        self.order_end.len()
    }

    fn try_get(&'a self, i: usize) -> Option<Self::Order> {
        if i < self.len() {
            let start: usize = if i == 0 { 0 } else { self.order_end[i - 1] };
            let end = self.order_end[i];
            Some(ChainIRef::new(self.elements, &self.orders[start..end]))
        } else {
            None
        }
    }

    fn push(&mut self, v: Self::Order) -> Result<(), AddError> {
        if v.elements != self.elements {
            return Err(AddError::Elements);
        }
        self.orders.reserve(v.len());
        let start = self.order_end.last().unwrap_or(&0);
        self.order_end.push(*start + v.len());
        self.orders.extend_from_slice(v.order);
        Ok(())
    }

    fn remove_element(&mut self, _target: usize) -> Result<(), &'static str> {
        todo!();
    }

    fn generate_uniform<R: rand::Rng>(&mut self, rng: &mut R, new_orders: usize) {
        if self.elements == 0 {
            return;
        }
        let v: &mut [usize] = &mut (0..self.elements).collect::<Vec<usize>>();
        self.orders.reserve(self.elements * new_orders);
        let range = Uniform::new(0, self.elements).unwrap();
        for _ in 0..new_orders {
            let elements = range.sample(rng) + 1;
            v.shuffle(rng);
            for &el in &v[..elements] {
                self.orders.push(el);
            }
            let start = self.order_end.last().unwrap_or(&0);
            self.order_end.push(*start + elements);
        }
    }
}

impl From<ChainDense> for ChainIDense {
    fn from(value: ChainDense) -> Self {
        let orders: usize = value.len();
        let order_end = (0..orders).map(|i| (i + 1) * value.elements).collect();
        ChainIDense { orders: value.orders, order_end, elements: value.elements }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};

    use super::*;
    use crate::{
        OrderOwned, OrderRef,
        chain::ChainI,
        tests::{BoundedArbitrary, std_rng},
    };

    /// Returns true if this struct is in a valid state, used for debugging.
    fn valid(cd: &ChainIDense) -> bool {
        let mut seen = vec![false; cd.elements];
        for v in cd.iter() {
            seen.fill(false);
            for &i in v.order {
                if i >= cd.elements || seen[i] {
                    return false;
                }
                seen[i] = true;
            }
        }
        for &o in &cd.order_end {
            if o > cd.orders.len() {
                return false;
            }
        }
        for o in cd.order_end.windows(2) {
            if o[0] > o[1] {
                return false;
            }
        }
        true
    }

    impl Arbitrary for ChainIDense {
        fn arbitrary(g: &mut Gen) -> Self {
            let (orders_count, elements): (usize, usize) = BoundedArbitrary::arbitrary(g);
            let mut orders = ChainIDense::new(elements);
            orders.generate_uniform(&mut std_rng(g), orders_count);
            orders
        }
    }

    #[quickcheck]
    fn generate(orders: ChainIDense) -> bool {
        valid(&orders)
    }

    #[quickcheck]
    fn iter_collect(orders: ChainIDense) -> bool {
        let orig = orders.clone();
        let parts: Vec<ChainI> = orders.iter().map(|x| x.to_owned()).collect();
        for i in 0..orders.len() {
            if parts[i].as_ref() != orig.get(i) {
                return false;
            }
        }
        true
    }
}
