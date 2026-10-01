pub struct BitReader<'a> {
    pub data: &'a [u8],
    pub byte_pos: usize,
    pub bit_offset: u8,
}
impl<'a> BitReader<'a> {
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            byte_pos: 0,
            bit_offset: 0,
        }
    }

    #[inline]
    pub fn read_bit(&mut self) -> u8 {
        let bit = (self.data[self.byte_pos] >> (7 - self.bit_offset)) & 1;
        self.bit_offset += 1;
        if self.bit_offset == 8 {
            self.bit_offset = 0;
            self.byte_pos += 1;
        }
        bit
    }

    #[inline]
    pub fn position(&self) -> usize {
        self.byte_pos * 8 + self.bit_offset as usize
    }
    #[inline]
    pub fn remaining(&self) -> usize {
        self.data.len() * 8 - self.position()
    }
    #[inline]
    pub fn skip(&mut self, n: usize) {
        let total = self.bit_offset as usize + n;
        self.byte_pos += total / 8;
        self.bit_offset = (total % 8) as u8;
    }
}
pub trait BitRead: Sized {
    fn read_from(reader: &mut BitReader<'_>, n: usize) -> Self;
}
macro_rules! impl_bit_read {
    ($($ty:ty),*) => {
        $(
            impl BitRead for $ty {
                #[inline]
                fn read_from(reader: &mut BitReader<'_>, n: usize) -> Self {
                    assert!(n <= <$ty>::BITS as usize);

                    let mut result = 0 as $ty;

                    for _ in 0..n {
                        result = (result << 1) | reader.read_bit() as $ty;
                    }

                    result
                }
            }
        )*
    };
}
impl_bit_read!(u8, u16, u32, u64);

macro_rules! impl_signed_bit_read {
    ($($ty:ty),*) => {
        $(
            impl BitRead for $ty {
                #[inline]
                fn read_from(reader: &mut BitReader<'_>, n: usize) -> Self {
                    assert!(n <= std::mem::size_of::<$ty>() * 8);

                    if n == 0 {
                        return 0;
                    }

                    let mut result = 0 as $ty;

                    for _ in 0..n {
                        result = result.wrapping_shl(1)
                            | reader.read_bit() as $ty;
                    }

                    let bits = std::mem::size_of::<$ty>() * 8;

                    if n < bits {
                        let sign_bit = (1 as $ty).wrapping_shl((n - 1) as u32);

                        if result & sign_bit != 0 {
                            result |= (!0 as $ty).wrapping_shl(n as u32);
                        }
                    }

                    result
                }
            }
        )*
    };
}

impl_signed_bit_read!(i8, i16, i32, i64);

impl<'a> BitReader<'a> {
    #[inline]
    pub fn read_bits<T: BitRead>(&mut self, n: usize) -> T {
        T::read_from(self, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u8() {
        let data = [0b1010_1100];
        let mut reader = BitReader::new(&data);
        let val: u8 = reader.read_bits(8);
        assert_eq!(val, 0b1010_1100);
        assert_eq!(reader.position(), 8);
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_read_u16() {
        let data = [0x12, 0x34];
        let mut reader = BitReader::new(&data);
        let val: u16 = reader.read_bits(16);
        assert_eq!(val, 0x1234);
        assert_eq!(reader.position(), 16);
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_read_u32() {
        let data = [0x12, 0x34, 0x56, 0x78];
        let mut reader = BitReader::new(&data);
        let val: u32 = reader.read_bits(32);
        assert_eq!(val, 0x1234_5678);
        assert_eq!(reader.position(), 32);
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_read_u64() {
        let data = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0];
        let mut reader = BitReader::new(&data);
        let val: u64 = reader.read_bits(64);
        assert_eq!(val, 0x1234_5678_9abc_def0);
        assert_eq!(reader.position(), 64);
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_read_i8() {
        let data = [0b1111_1111];
        let mut reader = BitReader::new(&data);
        let val: i8 = reader.read_bits(8);
        assert_eq!(val, -1);
        assert_eq!(reader.position(), 8);
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_read_i16() {
        let data = [0xff, 0xfe];
        let mut reader = BitReader::new(&data);
        let val: i16 = reader.read_bits(16);
        assert_eq!(val, -2);
        assert_eq!(reader.position(), 16);
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_read_i32() {
        let data = [0xff, 0xff, 0xff, 0xfe];
        let mut reader = BitReader::new(&data);
        let val: i32 = reader.read_bits(32);
        assert_eq!(val, -2);
        assert_eq!(reader.position(), 32);
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_read_i64() {
        let data = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe];
        let mut reader = BitReader::new(&data);
        let val: i64 = reader.read_bits(64);
        assert_eq!(val, -2);
        assert_eq!(reader.position(), 64);
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_read_i8_partial() {
        let data = [0b1111_0000];
        let mut reader = BitReader::new(&data);
        let val: i8 = reader.read_bits(4);
        assert_eq!(val, -1);
        assert_eq!(reader.position(), 4);
    }

    #[test]
    fn test_read_i8_partial_positive() {
        let data = [0b0111_0000];
        let mut reader = BitReader::new(&data);
        let val: i8 = reader.read_bits(4);
        assert_eq!(val, 7);
        assert_eq!(reader.position(), 4);
    }
    #[test]
    fn test_read_i8_partial_min() {
        let data = [0b1000_0000];
        let mut reader = BitReader::new(&data);
        let val: i8 = reader.read_bits(4);
        assert_eq!(val, -8);
        assert_eq!(reader.position(), 4);
    }
    #[test]
    fn test_read_i16_partial() {
        let data = [0b1111_1111, 0b1000_0000];
        let mut reader = BitReader::new(&data);
        let val: i16 = reader.read_bits(12);
        assert_eq!(val, -8);
        assert_eq!(reader.position(), 12);
    }
    #[test]
    fn test_read_i16_partial_positive() {
        let data = [0b0111_1111, 0b1000_0000];
        let mut reader = BitReader::new(&data);
        let val: i16 = reader.read_bits(12);
        assert_eq!(val, 2040);
        assert_eq!(reader.position(), 12);
    }

    #[test]
    fn test_read_i32_partial() {
        let data = [0b1111_1111, 0b1111_1111, 0b1111_0000];
        let mut reader = BitReader::new(&data);
        let val: i32 = reader.read_bits(20);
        assert_eq!(val, -1);
        assert_eq!(reader.position(), 20);
    }

    #[test]
    fn test_read_i64_partial() {
        let data = [
            0b1111_1111,
            0b1111_1111,
            0b1111_1111,
            0b1111_1111,
            0b1111_1000,
        ];
        let mut reader = BitReader::new(&data);
        let val: i64 = reader.read_bits(40);
        assert_eq!(val, -8);
        assert_eq!(reader.position(), 40);
    }

    #[test]
    fn test_read_signed_zero_bits() {
        let data = [0xff];
        let mut reader = BitReader::new(&data);
        let val: i8 = reader.read_bits(0);
        assert_eq!(val, 0);
        assert_eq!(reader.position(), 0);
        assert_eq!(reader.remaining(), 8);
    }
    #[test]
    fn test_read_across_bytes() {
        let data = [0b1010_1100, 0b0110_1001];
        let mut reader = BitReader::new(&data);

        // 101 01100 011 -> 10101100011
        let val: u16 = reader.read_bits(11);

        assert_eq!(val, 0b1010_1100_011);
        assert_eq!(reader.position(), 11);
    }

    #[test]
    fn test_read_multiple_values() {
        let data = [0b1011_0011, 0b0101_1110];
        let mut reader = BitReader::new(&data);

        let a: u8 = reader.read_bits(4);
        let b: u8 = reader.read_bits(4);
        let c: u8 = reader.read_bits(8);

        assert_eq!(a, 0b1011);
        assert_eq!(b, 0b0011);
        assert_eq!(c, 0b0101_1110);

        assert_eq!(reader.position(), 16);
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_skip() {
        let data = [0b1011_0011, 0b0101_1110];
        let mut reader = BitReader::new(&data);

        reader.skip(4);

        assert_eq!(reader.position(), 4);

        let val: u8 = reader.read_bits(4);
        assert_eq!(val, 0b0011);

        reader.skip(4);

        assert_eq!(reader.position(), 12);

        let val: u8 = reader.read_bits(4);
        assert_eq!(val, 0b1110);
    }
}
