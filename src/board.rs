use std::ops::BitAnd;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Bitboard(u64);

impl Bitboard {
    pub fn empty(&self) -> Self {
        Bitboard(0)
    }

    pub fn from_u64(value: u64) -> Self {
        debug_assert!(value <= 64);
        Bitboard(value)
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }

    pub fn set(&mut self, index: u8) {
        debug_assert!(index <= 64);
        self.0 |= 1u64 << index;
    }

    pub fn clear(&mut self, index: u8) {
        debug_assert!(index <= 64);
        self.0 &= !(1u64 << index);
    }

    pub fn is_set(&self, index: u8) -> bool {
        debug_assert!(index <= 64);
        (self.0 & (1u64 << index)) != 0
    }

    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Bitboard(0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        assert_eq!(2 + 2, 4);
    }
}