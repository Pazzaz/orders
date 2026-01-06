use crate::partial_order;

pub mod binary;
pub mod cardinal;
pub mod chain;
pub mod specific;
pub mod tied;

pub trait Order {
    /// The number of elements that can be in this order.
    fn elements(&self) -> usize;

    /// The number of elements currently part of this order.
    fn len(&self) -> usize;

    /// Shorthand for `self.len() == 0`
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn to_partial(self) -> partial_order::PartialOrder;
}

pub trait OrderOwned<'a> {
    type Ref;
    fn as_ref(&'a self) -> Self::Ref;
}

pub trait OrderRef {
    type Owned;
    fn to_owned(self) -> Self::Owned;
}
