mod binary;
mod cardinal;
mod specific;
mod strict;
mod tied;

pub use binary::BinaryDense;
pub use cardinal::CardinalDense;
pub use specific::SpecificDense;
pub use strict::{ChainDense, TotalDense};
pub use tied::{TiedDense, TiedIDense};

#[derive(Debug)]
pub enum AddError {
    Elements,
    Alloc,
}
