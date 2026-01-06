use std::cmp::Ordering;

use rand::{
    distr::{Bernoulli, Distribution},
    seq::{IndexedRandom, SliceRandom},
};

use crate::{
    collections::{AddError, CardinalDense, DenseOrders, SpecificDense, TotalDense},
    orders::tied::TiedRef,
};

/// TOC - Orders with Ties - Complete List
///
/// A packed list of complete orders with ties, with related methods.
#[derive(Debug)]
pub struct TiedDense {
    // Has length orders_count * elements
    pub(crate) orders: Vec<usize>,

    // Says if a value is tied with the next value.
    // Has length orders_count * (elements - 1)
    pub(crate) ties: Vec<bool>,
    pub(crate) elements: usize,
}

impl Clone for TiedDense {
    fn clone(&self) -> Self {
        Self { orders: self.orders.clone(), ties: self.ties.clone(), elements: self.elements }
    }

    fn clone_from(&mut self, source: &Self) {
        self.orders.clone_from(&source.orders);
        self.ties.clone_from(&source.ties);
        self.elements = source.elements;
    }
}

impl TiedDense {
    pub fn new(elements: usize) -> Self {
        TiedDense { orders: Vec::new(), ties: Vec::new(), elements }
    }

    pub fn iter(&self) -> impl Iterator<Item = TiedRef<'_>> {
        (0..self.len()).map(|i| self.get(i))
    }

    /// Pick a winning element from each ordering, randomly from their highest
    /// ranked (tied) elements.
    pub fn to_specific_using<R: rand::Rng>(self, rng: &mut R) -> SpecificDense {
        let elements = self.elements;
        let mut orders: SpecificDense =
            self.iter().map(|v| *v.winners().choose(rng).unwrap()).collect();

        orders.add_elements(orders.elements - elements);
        orders
    }
}

impl<'a> DenseOrders<'a> for TiedDense {
    type Order = TiedRef<'a>;

    fn elements(&self) -> usize {
        self.elements
    }

    fn len(&self) -> usize {
        if self.elements == 0 { 0 } else { self.orders.len() / self.elements }
    }

    fn push(&mut self, v: Self::Order) -> Result<(), AddError> {
        let order = v.order();
        let tie = v.tied();
        if order.len() != self.elements && self.elements != 0 {
            return Err(AddError::Elements);
        }

        self.orders.try_reserve(order.len() * self.elements).map_err(|_| AddError::Alloc)?;
        self.ties.try_reserve(tie.len() * (self.elements - 1)).map_err(|_| AddError::Alloc)?;

        self.orders.extend_from_slice(order);
        self.ties.extend_from_slice(tie);
        Ok(())
    }

    fn try_get(&'a self, i: usize) -> Option<Self::Order> {
        if i < self.len() {
            let start = i * self.elements;
            let end = (i + 1) * self.elements;
            Some(TiedRef::new(&self.orders[start..end], &self.ties[(start - i)..(end - i - 1)]))
        } else {
            None
        }
    }

    fn remove_element(&mut self, target: usize) -> Result<(), &'static str> {
        if self.elements <= target {
            return Err("Element not in collection");
        }
        if self.elements == 1 {
            self.orders.clear();
            self.ties.clear();
            self.elements = 0;
        } else if self.len() == 0 {
            self.elements -= 1;
        } else {
            // The len will not change
            let len = self.len();
            let elements_old = self.elements;
            let elements_new = self.elements - 1;
            for i in 0..self.len() {
                let mut skipped = None;
                for j in 0..elements_old {
                    let el = self.orders[i * elements_old + j];
                    let out = match target.cmp(&el) {
                        Ordering::Less => el,
                        Ordering::Equal => {
                            debug_assert!(skipped.is_none());
                            skipped = Some(j);
                            continue;
                        }
                        Ordering::Greater => el - 1,
                    };
                    if skipped.is_none() {
                        self.orders[i * elements_new + j] = out;
                    } else {
                        self.orders[i * elements_new + j - 1] = out;
                    }
                }
                if let Some(removed) = skipped {
                    let start_old = i * (elements_old - 1);
                    let end_old = (i + 1) * (elements_old - 1);
                    let start_new = i * (elements_new - 1);
                    let end_new = (i + 1) * (elements_new - 1);
                    if removed == 0 {
                        self.ties.copy_within((start_old + 1)..end_old, start_new);
                    } else if removed == (elements_old - 1) {
                        self.ties.copy_within(start_old..(end_old - 1), start_new);
                    } else {
                        debug_assert!(0 < removed && removed < (elements_old - 1));
                        // TODO: This may be wrong...
                        let pre = self.ties[start_old..end_old][removed - 1];
                        let next = self.ties[start_old..end_old][removed];
                        self.ties.copy_within(start_old..(start_old + removed - 1), start_new);
                        self.ties.copy_within((start_old + removed)..end_old, start_new);
                        self.ties[start_new..end_new][removed - 1] = pre && next;
                    }
                } else {
                    unreachable!();
                }
            }
            self.orders.truncate(len * elements_new);
            self.ties.truncate(len * (elements_new - 1));
            self.elements = elements_new;
        }
        Ok(())
    }

    fn generate_uniform<R: rand::Rng>(&mut self, rng: &mut R, new_orders: usize) {
        if self.elements == 0 {
            return;
        }
        let v: &mut [usize] = &mut (0..self.elements).collect::<Vec<usize>>();
        self.orders.reserve(new_orders * self.elements);
        self.ties.reserve(new_orders * (self.elements - 1));
        let dist = Bernoulli::new(0.5).unwrap();
        for _ in 0..new_orders {
            v.shuffle(rng);
            for &el in &*v {
                self.orders.push(el);
            }

            for _ in 0..(self.elements - 1) {
                let b = dist.sample(rng);
                self.ties.push(b);
            }
        }
    }
}

impl TryFrom<TiedDense> for CardinalDense {
    type Error = &'static str;

    /// Convert each ordering to a cardinal order, with the highest rank
    /// elements receiving a score of `self.elements`.
    ///
    /// Returns `Err` if it failed to allocate.
    fn try_from(value: TiedDense) -> Result<Self, Self::Error> {
        let mut orders: Vec<usize> = Vec::new();
        orders.try_reserve_exact(value.elements * value.len()).or(Err("Could not allocate"))?;
        let max = value.elements - 1;
        let mut new_order = vec![0; value.elements];
        for order in value.iter() {
            for (i, group) in order.iter_groups().enumerate() {
                for &c in group {
                    debug_assert!(max >= i);
                    new_order[c] = max - i;
                }
            }
            // `order` is a ranking of all elements, so `new_order` will be different
            // between iterations.
            orders.extend(&new_order);
        }
        Ok(CardinalDense { orders, elements: value.elements, min: 0, max })
    }
}

impl From<TotalDense> for TiedDense {
    fn from(value: TotalDense) -> Self {
        let orders: usize = value.len();
        TiedDense {
            orders: value.orders,
            ties: vec![false; (value.elements - 1) * orders],
            elements: value.elements,
        }
    }
}

impl<'a> FromIterator<TiedRef<'a>> for Option<TiedDense> {
    /// Returns [`None`] if any orders have a different number of
    /// elements, or the iterator is empty.
    fn from_iter<T: IntoIterator<Item = TiedRef<'a>>>(iter: T) -> Self {
        let mut ii = iter.into_iter();
        if let Some(first_value) = ii.next() {
            let elements = first_value.elements();
            let mut out = TiedDense::new(elements);
            out.push(first_value).unwrap();
            for v in ii {
                if v.elements() != elements {
                    return None;
                }
                out.push(v).unwrap();
            }
            Some(out)
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, Gen};

    use super::*;
    use crate::tests::std_rng;

    /// Returns true if this struct is in a valid state, used for debugging.
    fn valid(td: &TiedDense) -> bool {
        if td.orders.len() != td.len() * td.elements
            || td.ties.len() != td.len() * td.elements.saturating_sub(1)
        {
            return false;
        }
        let mut seen = vec![false; td.elements];
        for order in td.iter() {
            seen.fill(false);
            if order.order().len() != td.elements || order.tied().len() != td.elements - 1 {
                return false;
            }
            for &i in order.order() {
                if i >= td.elements || seen[i] {
                    return false;
                }
                seen[i] = true;
            }
        }
        true
    }

    impl Arbitrary for TiedDense {
        fn arbitrary(g: &mut Gen) -> Self {
            let (mut orders_count, mut elements): (usize, usize) = Arbitrary::arbitrary(g);

            // `Arbitrary` for numbers will generate "problematic" examples such as
            // `usize::max_value()` and `usize::min_value()` but we'll use them to
            // allocate vectors so we'll limit them.
            elements = elements % g.size();
            orders_count = if elements != 0 { orders_count % g.size() } else { 0 };

            let mut orders = TiedDense::new(elements);
            orders.generate_uniform(&mut std_rng(g), orders_count);
            orders
        }
    }

    #[quickcheck]
    fn generate(orders: TiedDense) -> bool {
        valid(&orders)
    }

    #[test]
    fn collect_empty() {
        let v: Vec<TiedRef> = Vec::new();
        let res: Option<TiedDense> = v.into_iter().collect();
        assert!(res.is_none());
    }
}
