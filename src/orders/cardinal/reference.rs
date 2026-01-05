use crate::{OrderRef, binary::Binary, cardinal::Cardinal};

pub struct CardinalRef<'a> {
    pub(crate) values: &'a [usize],
}

impl<'a> CardinalRef<'a> {
    pub fn new(s: &'a [usize]) -> Self {
        CardinalRef { values: s }
    }

    /// Returns the number of elements in the order
    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn values(&self) -> &'a [usize] {
        self.values
    }

    /// Convert to binary order, where any value less than `cutoff` becomes
    /// `false` and larger becomes `true`.
    pub fn to_binary(&self, cutoff: usize) -> Binary {
        let values = self.values.iter().map(|x| *x >= cutoff).collect();
        Binary::new(values)
    }
}

impl OrderRef for CardinalRef<'_> {
    type Owned = Cardinal;

    fn to_owned(self) -> Self::Owned {
        Cardinal { values: self.values.to_owned() }
    }
}
