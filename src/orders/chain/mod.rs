//! # Chains
//!
//! A chain is an order of all elements where every element is comparable,
//! ordered from largest to smallest. Chains are also called total orders,
//! linear orders or chain orders.
//!
//! |               | **Complete**  | **Incomplete** |
//! | ------------- | ------------- | -------------  |
//! | **Owned**     | [`Chain`]     | [`ChainI`]     |
//! | **Reference** | [`ChainRef`]  | [`ChainIRef`]  |
//!
//! For collections of chains, see
//! [`ChainDense`](crate::collections::ChainDense) and
//! [`ChainIDense`](crate::collections::ChainIDense).
//!
//! # See also
//! - [Wikipedia][wp]
//! - [nLab][nlab]
//!
//! [wp]:   https://en.wikipedia.org/wiki/Total_order
//! [nlab]: https://ncatlab.org/nlab/show/total+order

mod complete;
mod incomplete;

pub use complete::{Chain, ChainRef};
pub use incomplete::{ChainI, ChainIRef};
