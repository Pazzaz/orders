//! # Tied orders
//!
//! A tied order is an order where every element is comparable, ordered from
//! largest to smallest, where some elements can be tied.
//!
//! |               | **Complete**  | **Incomplete** |
//! | ------------- | ------------- | -------------  |
//! | **Owned**     | [`Tied`]     | [`TiedI`]     |
//! | **Reference** | [`TiedRef`]  | [`TiedIRef`]  |
//!
//! For collections of tied orders, see
//! [`TiedDense`](crate::collections::TiedDense) and
//! [`TiedIDense`](crate::collections::TiedIDense).

mod complete;
mod incomplete;
mod split_ref;

pub use complete::{Tied, TiedRef};
pub use incomplete::{GroupIterator, TiedI, TiedIRef};
