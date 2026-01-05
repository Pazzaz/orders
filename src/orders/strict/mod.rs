//! # Total orders
//!
//! A [total order][wp] is an order of all elements where every element is
//! comparable, ordered from largest to smallest. A chain is like a total order,
//! but only orders a subset of all elements.
//!
//! - [`Total`], an owned total order.
//! - [`TotalRef`], reference to a `Total`.
//! - [`Chain`], an owned total order of a subset of all elements.
//! - [`ChainRef`], reference to a `Chain`.
//!
//! For collections of total orders, see
//! [`TotalDense`](crate::collections::TotalDense) and
//! [`ChainDense`](crate::collections::ChainDense).
//!
//! [wp]: https://en.wikipedia.org/wiki/Total_order

mod complete;
mod incomplete;

pub use complete::{Total, TotalRef};
pub use incomplete::{Chain, ChainRef};
