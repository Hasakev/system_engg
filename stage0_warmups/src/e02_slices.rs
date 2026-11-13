//! e02: slices.
//!
//! A slice reference is a "fat pointer": an address and a length. It owns
//! nothing; it is a window onto memory someone else owns.
//!
//! ```text
//!   let v = vec![10, 20, 30, 40, 50];
//!   let w = &v[1..4];
//!
//!   stack                      heap
//!   v: | ptr | len 5 | cap 5 |  -> | 10 | 20 | 30 | 40 | 50 |
//!   w: | ptr | len 3 |  ---------------^
//! ```
//!
//! `&str` is the same thing over UTF-8 bytes. Returning a sub-slice costs two
//! words and no allocation, which is why the functions here return slices
//! instead of new `String`s or `Vec`s.
//!
//! `find_subslice` comes back in your `grep`, so make it one you trust.
//!
//! Before you code, write answers in ANSWERS.md:
//! 1. What is `size_of::<&[u8]>()` on a 64-bit machine, and why?
//! 2. Why does `split_halves` need `split_at_mut` (or unsafe) when
//!    `(&mut xs[..mid], &mut xs[mid..])` describes non-overlapping memory?

/// The text up to (not including) the first ASCII space, or all of `s`.
pub fn first_word(s: &str) -> &str {
    todo!()
}

/// Reverses the slice in place. Write the loop yourself; no `.reverse()`.
pub fn reverse_in_place(xs: &mut [u8]) {
    todo!()
}

/// Index of the first occurrence of `needle` in `hay`.
/// An empty needle matches at 0.
pub fn find_subslice(hay: &[u8], needle: &[u8]) -> Option<usize> {
    todo!()
}

/// Splits into two mutable halves. The left half has `len / 2` elements.
pub fn split_halves(xs: &mut [i32]) -> (&mut [i32], &mut [i32]) {
    todo!()
}

/// The sub-slice with leading and trailing ASCII whitespace removed.
pub fn trim_ascii_ws(s: &[u8]) -> &[u8] {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_basics() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
        assert_eq!(first_word(" lead"), "");
    }

    #[test]
    fn first_word_borrows_from_input() {
        let s = String::from("alpha beta");
        assert_eq!(first_word(&s).as_ptr(), s.as_ptr());
    }

    #[test]
    fn reverse_handles_all_lengths() {
        let mut empty: [u8; 0] = [];
        reverse_in_place(&mut empty);
        let mut one = [7];
        reverse_in_place(&mut one);
        assert_eq!(one, [7]);
        let mut even = [1, 2, 3, 4];
        reverse_in_place(&mut even);
        assert_eq!(even, [4, 3, 2, 1]);
        let mut odd = *b"abcde";
        reverse_in_place(&mut odd);
        assert_eq!(&odd, b"edcba");
    }

    #[test]
    fn find_subslice_cases() {
        assert_eq!(find_subslice(b"hello world", b"o w"), Some(4));
        assert_eq!(find_subslice(b"aaab", b"aab"), Some(1));
        assert_eq!(find_subslice(b"abc", b"abc"), Some(0));
        assert_eq!(find_subslice(b"abc", b"c"), Some(2));
        assert_eq!(find_subslice(b"abc", b"abcd"), None);
        assert_eq!(find_subslice(b"abc", b"x"), None);
        assert_eq!(find_subslice(b"abc", b""), Some(0));
        assert_eq!(find_subslice(b"", b""), Some(0));
        assert_eq!(find_subslice(b"", b"a"), None);
    }

    #[test]
    fn split_halves_gives_two_live_mutable_views() {
        let mut xs = [1, 2, 3, 4, 5];
        let (left, right) = split_halves(&mut xs);
        assert_eq!((left.len(), right.len()), (2, 3));
        left[0] = 10;
        right[0] = 30;
        assert_eq!(xs, [10, 2, 30, 4, 5]);
    }

    #[test]
    fn trim_cases() {
        assert_eq!(trim_ascii_ws(b"  hi there\t\n"), b"hi there");
        assert_eq!(trim_ascii_ws(b"tight"), b"tight");
        assert_eq!(trim_ascii_ws(b" \t\n"), b"");
        assert_eq!(trim_ascii_ws(b""), b"");
    }

    #[test]
    fn trim_returns_a_window_not_a_copy() {
        let buf = b"  xyz ".to_vec();
        let out = trim_ascii_ws(&buf);
        assert_eq!(out.as_ptr(), buf[2..].as_ptr());
    }
}
