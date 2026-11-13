//! e03: bits, bytes and byte order.
//!
//! Hardware, file formats and network protocols are all defined in bits and
//! bytes, so this is the vocabulary of every later stage.
//!
//! ```text
//!   the u16 0xD123, bit 15 on the left:
//!
//!     1101 0001 0010 0011
//!     \__/ \__/ \__/ \__/
//!      D    1    2    3        four nibbles
//!
//!   the u32 0x11223344 in memory, lowest address on the left:
//!
//!     big-endian:     11 22 33 44    (network order)
//!     little-endian:  44 33 22 11    (x86, ARM, RISC-V)
//! ```
//!
//! Use shifts and masks (`<<`, `>>`, `&`, `|`, `!`). The standard library has
//! `from_be_bytes` and friends; write these by hand once, then use std forever.
//!
//! `align_up` returns in e09 and in the allocator stage. `nibbles` is how the
//! CHIP-8 emulator decodes instructions.
//!
//! Before you code, write answers in ANSWERS.md:
//! 1. For a power of two `a`, what does `a - 1` look like in binary?
//!    How does that make `align_up` a two-operation function?
//! 2. What does `1u8 << 8` do in a debug build? In a release build?

/// Whether bit `i` of `x` is set. Bit 0 is the least significant. `i < 8`.
pub fn get_bit(x: u8, i: u8) -> bool {
    todo!()
}

/// `x` with bit `i` forced on or off. `i < 8`.
pub fn set_bit(x: u8, i: u8, on: bool) -> u8 {
    todo!()
}

/// The four 4-bit groups of `w`, most significant first.
/// `0xD123` becomes `[0xD, 0x1, 0x2, 0x3]`.
pub fn nibbles(w: u16) -> [u8; 4] {
    todo!()
}

/// Assembles a `u32` from four bytes in big-endian order.
pub fn u32_from_be(b: [u8; 4]) -> u32 {
    todo!()
}

/// Splits a `u32` into four bytes in little-endian order.
pub fn u32_to_le(x: u32) -> [u8; 4] {
    todo!()
}

/// Rounds `addr` up to the next multiple of `align`.
/// `align` is a power of two. No division, no loop.
pub fn align_up(addr: usize, align: usize) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bit_reads_each_position() {
        assert!(get_bit(0b0000_0001, 0));
        assert!(!get_bit(0b0000_0001, 1));
        assert!(get_bit(0b1000_0000, 7));
        assert!(!get_bit(0b0111_1111, 7));
    }

    #[test]
    fn set_bit_on_and_off() {
        assert_eq!(set_bit(0, 3, true), 0b0000_1000);
        assert_eq!(set_bit(0xFF, 3, false), 0b1111_0111);
        assert_eq!(set_bit(0b1000, 3, true), 0b1000);
        assert_eq!(set_bit(0, 7, true), 0x80);
        assert_eq!(set_bit(0, 0, false), 0);
    }

    #[test]
    fn nibbles_splits_high_to_low() {
        assert_eq!(nibbles(0xD123), [0xD, 0x1, 0x2, 0x3]);
        assert_eq!(nibbles(0x00E0), [0, 0, 0xE, 0]);
        assert_eq!(nibbles(0xFFFF), [0xF; 4]);
    }

    #[test]
    fn big_endian_matches_std() {
        for bytes in [[0x11, 0x22, 0x33, 0x44], [0xFF, 0, 0, 0], [0, 0, 0, 0xFF]] {
            assert_eq!(u32_from_be(bytes), u32::from_be_bytes(bytes));
        }
    }

    #[test]
    fn little_endian_matches_std() {
        for x in [0x1122_3344, 0xFF00_0000, 0x0000_00FF, u32::MAX, 0] {
            assert_eq!(u32_to_le(x), x.to_le_bytes());
        }
    }

    #[test]
    fn align_up_rounds_to_multiples() {
        assert_eq!(align_up(0, 8), 0);
        assert_eq!(align_up(1, 8), 8);
        assert_eq!(align_up(8, 8), 8);
        assert_eq!(align_up(9, 8), 16);
        assert_eq!(align_up(13, 1), 13);
        assert_eq!(align_up(4095, 4096), 4096);
        assert_eq!(align_up(4097, 4096), 8192);
    }
}
