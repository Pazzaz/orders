use super::Binary;
use crate::OrderRef;

pub struct BinaryRef<'a> {
    pub(crate) values: &'a [bool],
}

impl<'a> BinaryRef<'a> {
    pub fn new(v: &'a [bool]) -> Self {
        BinaryRef { values: v }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn values(&self) -> &'a [bool] {
        self.values
    }
}

impl OrderRef for BinaryRef<'_> {
    type Owned = Binary;

    fn to_owned(self) -> Self::Owned {
        Binary { values: self.values.to_vec() }
    }
}
