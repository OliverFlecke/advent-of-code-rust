#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BitSet(u64);

impl BitSet {
    pub fn insert(&mut self, k: usize) {
        self.0 |= 0b1 << k;
    }

    pub fn contains(&self, k: usize) -> bool {
        self.0 & (0b1 << k) != 0
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}
