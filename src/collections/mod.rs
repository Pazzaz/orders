mod binary;
mod cardinal;
mod specific;
mod strict;
mod tied;

pub use binary::BinaryDense;
pub use cardinal::CardinalDense;
use rand::Rng;
pub use specific::SpecificDense;
pub use strict::{ChainDense, TotalDense};
pub use tied::{TiedDense, TiedIDense};

// Lifetime needed because `Order` may be a reference which then needs a
// lifetime
pub trait DenseOrders<'a> {
    type Order;
    /// Number of elements
    fn elements(&self) -> usize;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn add(&mut self, v: Self::Order) -> Result<(), AddError>;

    fn try_get(&'a self, i: usize) -> Option<Self::Order>;

    fn get(&'a self, i: usize) -> Self::Order {
        self.try_get(i).unwrap()
    }

    /// Removes element from the orders, offsetting the other elements to
    /// take their place.
    fn remove_element(&mut self, target: usize) -> Result<(), &'static str>;

    /// Sample and add `new_orders` uniformly random orders for this format,
    /// using random numbers from `rng`.
    fn generate_uniform<R: Rng>(&mut self, rng: &mut R, new_orders: usize);
}

#[derive(Debug)]
pub enum AddError {
    Elements,
    Alloc,
}
