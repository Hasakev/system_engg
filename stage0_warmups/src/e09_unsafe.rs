//! e09: raw pointers and `unsafe`.
//!
//! `unsafe` does not switch the borrow checker off. It unlocks a few extra
//! operations, the main one being dereferencing a raw pointer (`*const T`,
//! `*mut T`), and makes you responsible for what the compiler normally
//! proves: the pointer is non-null, aligned, points at a live, initialised
//! `T`, and you are not breaking the aliasing rules.
//!
//! The habit to build: every `unsafe` block gets a `// SAFETY:` comment that
//! says why those conditions hold. If you cannot write the comment, the code
//! is not ready.
//!
//! The last exercise is a bump allocator, the simplest allocator there is:
//!
//! ```text
//!   buf, N = 16                         next
//!                                        v
//!   | A | A | A | . | B | B | B | B | . | . | . | . | . | . | . | . |
//!     0           3   4               8
//!
//!   alloc(3, 1) -> offset 0, next = 3
//!   alloc(4, 4) -> offset 3 is not a multiple of 4, skip to 4, next = 8
//!   reset()     -> next = 0, everything is free again
//! ```
//!
//! The picture uses offsets to keep it small. Real alignment is about the
//! address: `ptr as usize % align == 0`. The buffer itself can start at any
//! address, so align the address, not the offset. Your `align_up` from e03
//! does the rounding.
//!
//! Stage 2 replaces this with a real allocator that can free single blocks.
//!
//! Before you code, write answers in ANSWERS.md:
//! 1. `alloc` hands out pointers into `self.buf`. What happens to them if the
//!    `Bump` value is moved (returned from a function, pushed into a `Vec`)?
//! 2. Why does `alloc` return a raw pointer instead of `&mut [u8]`? Try
//!    writing the `&mut [u8]` version and calling it twice.

use crate::e03_bits::align_up;
use std::ptr::NonNull;

/// Exchanges the values behind two pointers using `ptr::read` and
/// `ptr::write`. No `ptr::swap`, no `mem::swap`.
///
/// # Safety
/// Both pointers must be valid for reads and writes, aligned, and point at
/// initialised values. They may be equal.
pub unsafe fn swap_raw<T>(a: *mut T, b: *mut T) {
    todo!()
}

/// Sums `len` consecutive `u32`s starting at `ptr`, using pointer arithmetic.
/// Do not build a slice.
///
/// # Safety
/// `ptr` must be aligned and valid for reading `len` elements.
pub unsafe fn sum_raw(ptr: *const u32, len: usize) -> u64 {
    todo!()
}

/// Reads a little-endian `u32` from `buf` starting at byte `offset`, or
/// `None` if fewer than four bytes remain. This is a safe function wrapping
/// an unsafe read: do the bounds check, then use `ptr::read_unaligned`.
pub fn read_u32_le_at(buf: &[u8], offset: usize) -> Option<u32> {
    todo!()
}

/// A bump allocator over an inline buffer of `N` bytes.
pub struct Bump<const N: usize> {
    buf: [u8; N],
    /// Offset of the first free byte.
    next: usize,
}

impl<const N: usize> Bump<N> {
    pub fn new() -> Self {
        todo!()
    }

    /// Bytes consumed so far, alignment padding included.
    pub fn used(&self) -> usize {
        todo!()
    }

    /// Reserves `size` bytes whose address is a multiple of `align` (a power
    /// of two). Returns `None`, changing nothing, if they do not fit.
    pub fn alloc(&mut self, size: usize, align: usize) -> Option<NonNull<u8>> {
        todo!()
    }

    /// Frees everything at once.
    pub fn reset(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_raw_swaps_plain_values() {
        let (mut a, mut b) = (1u64, 2u64);
        unsafe { swap_raw(&mut a, &mut b) };
        assert_eq!((a, b), (2, 1));
    }

    #[test]
    fn swap_raw_neither_leaks_nor_double_frees() {
        let mut a = String::from("alpha");
        let mut b = String::from("beta");
        unsafe { swap_raw(&mut a, &mut b) };
        assert_eq!((a.as_str(), b.as_str()), ("beta", "alpha"));
    }

    #[test]
    fn swap_raw_with_itself() {
        let mut a = String::from("solo");
        let p: *mut String = &mut a;
        unsafe { swap_raw(p, p) };
        assert_eq!(a, "solo");
    }

    #[test]
    fn sum_raw_adds_without_overflowing_u32() {
        let xs = [u32::MAX, u32::MAX, 2];
        let total = unsafe { sum_raw(xs.as_ptr(), xs.len()) };
        assert_eq!(total, 2 * u32::MAX as u64 + 2);
        assert_eq!(unsafe { sum_raw(xs.as_ptr(), 0) }, 0);
    }

    #[test]
    fn read_u32_le_at_any_offset() {
        let buf = [0xFF, 0x44, 0x33, 0x22, 0x11, 0xEE];
        assert_eq!(read_u32_le_at(&buf, 1), Some(0x1122_3344));
        assert_eq!(read_u32_le_at(&buf, 2), Some(0xEE11_2233));
        assert_eq!(read_u32_le_at(&buf, 3), None);
        assert_eq!(read_u32_le_at(&buf, usize::MAX), None);
        assert_eq!(read_u32_le_at(&[], 0), None);
    }

    #[test]
    fn bump_respects_alignment() {
        let mut bump = Bump::<128>::new();
        bump.alloc(1, 1).unwrap();
        for align in [2, 4, 8, 16] {
            let p = bump.alloc(1, align).unwrap();
            assert_eq!(p.as_ptr() as usize % align, 0, "align {align}");
        }
    }

    #[test]
    fn bump_blocks_do_not_overlap_and_are_writable() {
        let mut bump = Bump::<64>::new();
        let a = bump.alloc(8, 1).unwrap();
        let b = bump.alloc(8, 8).unwrap();
        assert!(b.as_ptr() as usize >= a.as_ptr() as usize + 8);
        unsafe {
            a.as_ptr().write_bytes(0xAA, 8);
            b.as_ptr().write_bytes(0xBB, 8);
            assert_eq!(*a.as_ptr().add(7), 0xAA);
            assert_eq!(*b.as_ptr(), 0xBB);
        }
        let lo = bump.buf.as_ptr() as usize;
        assert!(a.as_ptr() as usize >= lo && b.as_ptr() as usize + 8 <= lo + 64);
    }

    #[test]
    fn bump_runs_out_then_resets() {
        let mut bump = Bump::<16>::new();
        assert!(bump.alloc(17, 1).is_none());
        assert_eq!(bump.used(), 0, "a failed alloc must not consume space");
        assert!(bump.alloc(16, 1).is_some());
        assert_eq!(bump.used(), 16);
        assert!(bump.alloc(1, 1).is_none());
        bump.reset();
        assert_eq!(bump.used(), 0);
        assert!(bump.alloc(1, 1).is_some());
    }

    #[test]
    fn bump_survives_absurd_sizes() {
        let mut bump = Bump::<16>::new();
        bump.alloc(1, 1).unwrap();
        assert!(bump.alloc(usize::MAX, 1).is_none());
        assert_eq!(bump.used(), 1);
    }
}
