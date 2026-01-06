//! This is a library of different representations of orders. The most general
//! order type is [`PartialOrder`](partial_order::PartialOrder), but we can
//! represent orders more effectively if we use a type for a smaller set of
//! orders. Every order is finite, and stores how many elements the ordered set
//! has&nbsp;(see [`Order::elements`]).
//!
//! The different types of orders are
//! - [`Binary`](binary), a ranked order where every element either has a high
//!   rank or a low rank.
//! - [`Cardinal`](cardinal), a ranked order where every element is assigned
//!   some number.
//! - [`PartialOrder`](partial_order), a partial order
//! - [`Chain`](chain), a linear order containing every element.
//! - [`Tied`](tied), a linear order containing every element, where some
//!   elements can be tied.
//!
//! There are also variants of the orders which don't store all elements. Stored
//! elements are considered higher in the poset than non-stored elements.
//!
//! There are also custom [`collections`] of orders. These are more effective
//! than just using a [`Vec`] of orders, as the orders themselves often contain
//! a `Vec`. By using custom containers it's possible to store them in a more
//! compact form and avoid nested containers.

#![feature(test)]
extern crate test;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod collections;
mod orders;
pub mod partial_order;

pub use orders::*;

fn pairwise_lt(v: &[usize]) -> bool {
    if v.len() >= 2 {
        for i in 0..(v.len() - 1) {
            if v[i] >= v[i + 1] {
                return false;
            }
        }
    }
    true
}

fn get_order<T: Ord>(v: &[T], reverse: bool) -> Vec<usize> {
    if v.is_empty() {
        return Vec::new();
    } else if v.len() == 1 {
        return vec![0];
    }

    let mut tmp: Vec<(usize, &T)> = Vec::with_capacity(v.len());
    for (i, el) in v.iter().enumerate() {
        tmp.push((i, el));
    }
    tmp.sort_by(|a, b| (*a.1).cmp(b.1));
    if reverse {
        tmp.reverse();
    }
    let mut out = vec![0; v.len()];
    if let Some((b, bs)) = tmp.split_first_mut() {
        let mut current: &T = b.1;
        let mut i: usize = 0;
        for x in bs.iter_mut() {
            if *x.1 != *current {
                current = x.1;
                i += 1;
            }
            out[x.0] = i;
        }
    }
    out
}

// Sort two arrays, sorted according to the values in `b`.
// Uses insertion sort
pub(crate) fn sort_using<A, B>(a: &mut [A], b: &mut [B])
where
    B: PartialOrd,
{
    assert!(a.len() == b.len());
    let mut i: usize = 1;
    while i < b.len() {
        let mut j = i;
        while j > 0 && b[j - 1] > b[j] {
            a.swap(j, j - 1);
            b.swap(j, j - 1);
            j -= 1;
        }
        i += 1;
    }
}

use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

fn unique_and_bounded(elements: usize, order: &[usize]) -> bool {
    for (i, &a) in order.iter().enumerate() {
        if a >= elements {
            return false;
        }
        for (j, &b) in order.iter().enumerate() {
            if i == j {
                continue;
            }
            if a == b {
                return false;
            }
        }
    }
    true
}

pub(crate) fn add_bool<R: Rng>(rng: &mut R, v: &mut Vec<bool>, n: usize) {
    v.extend(<_ as Distribution<bool>>::sample_iter::<&mut R>(StandardUniform, rng).take(n));
}

#[cfg(test)]
mod tests {
    use std::mem;

    use quickcheck::{Arbitrary, Gen};
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    // `Gen` contains a rng, but it's a private member so this method is used to get
    // a standard rng generated from `Gen`
    pub fn std_rng(g: &mut Gen) -> StdRng {
        let mut seed = [0u8; 32];
        for i in 0..32 {
            seed[i] = Arbitrary::arbitrary(g);
        }
        StdRng::from_seed(seed)
    }

    #[quickcheck]
    fn sort_using_arbitrary(a: Vec<usize>, b: Vec<usize>) -> bool {
        let mut aa = a;
        let mut bb = b;
        if bb.len() < aa.len() {
            mem::swap(&mut aa, &mut bb);
        }
        let bbb = &mut bb[..aa.len()];
        sort_using(&mut aa, bbb);
        bbb.is_sorted()
    }

    #[test]
    fn sort_using_empty() {
        sort_using::<usize, usize>(&mut [], &mut []);
    }

    #[test]
    #[should_panic]
    fn sort_using_wrong0() {
        sort_using::<usize, usize>(&mut [], &mut [5]);
    }

    #[test]
    #[should_panic]
    fn sort_using_wrong1() {
        sort_using::<usize, usize>(&mut [1, 0], &mut [5]);
    }

    #[test]
    #[should_panic]
    fn sort_using_wrong2() {
        sort_using::<usize, usize>(&mut [6], &mut []);
    }

    #[test]
    #[should_panic]
    fn sort_using_wrong3() {
        sort_using::<usize, usize>(&mut [5], &mut [5, 0]);
    }
}
