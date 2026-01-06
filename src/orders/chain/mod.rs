//! # Total orders
//!
//! A [total order][wp] is an order of all elements where every element is
//! comparable, ordered from largest to smallest. A chain is like a total order,
//! but only orders a subset of all elements.
//!
//! - [`Chain`], an owned total order.
//! - [`ChainRef`], reference to a `Chain`.
//! - [`ChainI`], an owned total order of a subset of all elements.
//! - [`ChainIRef`], reference to a `ChainI`.
//!
//! For collections of total orders, see
//! [`ChainDense`](crate::collections::ChainDense) and
//! [`ChainIDense`](crate::collections::ChainIDense).
//!
//! [wp]: https://en.wikipedia.org/wiki/Total_order

mod complete;
mod incomplete;

pub use complete::{Chain, ChainRef};
pub use incomplete::{ChainI, ChainIRef};
