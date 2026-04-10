use std::fmt;
use std::fmt::Formatter;
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
use crate::position::Position;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Bitboard(u64);

impl Bitboard {
    fn validate_index(index: u8) {
        assert!(index < 64, "bit index out of range: {index}");
    }

    pub fn empty() -> Self {
        Bitboard(0)
    }

    pub fn from_u64(value: u64) -> Self {
        Bitboard(value)
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }

    pub fn set(&mut self, index: u8) {
        Self::validate_index(index);
        self.0 |= 1u64 << index;
    }

    pub fn clear(&mut self, index: u8) {
        Self::validate_index(index);
        self.0 &= !(1u64 << index);
    }

    pub fn is_set(&self, index: u8) -> bool {
        Self::validate_index(index);
        (self.0 & (1u64 << index)) != 0
    }

    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn is_not_empty(&self) -> bool {
        self.0 != 0
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

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitXor for Bitboard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl Shl<u8> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<u8> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in (0..Position::HEIGHT).rev() {
            for x in (0..Position::WIDTH).rev() {
                let index = y * Position::WIDTH + x;
                write!(f, "{} ", if self.is_set(index as u8) {"1"} else {"0"})?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Bitboard;

    #[test]
    fn default_is_empty() {
        let b = Bitboard::default();
        assert_eq!(b.to_u64(), 0);
        assert_eq!(b.count(), 0);
    }

    #[test]
    fn set_and_is_set() {
        let mut b = Bitboard::default();

        b.set(0);
        assert_eq!(b.count(), 1);
        assert!(b.is_set(0));

        b.set(63);
        assert_eq!(b.count(), 2);
        assert!(b.is_set(63));
    }

    #[test]
    fn clear_unsets_bits() {
        let mut b = Bitboard::default();

        b.set(1);
        assert!(b.is_set(1));

        b.clear(1);
        assert!(!b.is_set(1));
        assert_eq!(b.count(), 0);
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds() {
        let mut b = Bitboard::default();

        b.set(64);
    }

    #[test]
    fn bit_and() {
        let a = Bitboard::from_u64(0b1101);
        let b = Bitboard::from_u64(0b1011);

        let result = a & b;

        assert_eq!(result.to_u64(), 0b1001);
    }

    #[test]
    fn bit_or() {
        let a = Bitboard::from_u64(0b1101);
        let b = Bitboard::from_u64(0b1011);

        let result = a | b;

        assert_eq!(result.to_u64(), 0b1111);
    }

    #[test]
    fn bit_xor() {
        let a = Bitboard::from_u64(0b1101);
        let b = Bitboard::from_u64(0b1011);

        let result = a ^ b;

        assert_eq!(result.to_u64(), 0b0110);
    }

    #[test]
    fn bit_not() {
        let b = Bitboard::from_u64(0);

        let result = !b;

        assert_eq!(result.to_u64(), !0u64);
    }

    #[test]
    fn commutativity() {
        let a = Bitboard::from_u64(0xF0F0);
        let b = Bitboard::from_u64(0x0FF0);

        assert_eq!(a & b, b & a);
        assert_eq!(a | b, b | a);
        assert_eq!(a ^ b, b ^ a);
    }

    #[test]
    fn de_morgan() {
        let a = Bitboard::from_u64(0xF0F0);
        let b = Bitboard::from_u64(0x0FF0);

        assert_eq!(!(a & b), !a | !b);
    }
}