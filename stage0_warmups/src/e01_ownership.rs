//! e01: ownership, moves and borrows.
//!
//! A `String` is three words on the stack that own a buffer on the heap:
//!
//! ```text
//!   stack                         heap
//!   +-----+-----+-----+           +---+---+---+---+---+---+---+---+
//! s | ptr | len | cap |    ptr -> | h | e | l | l | o | ? | ? | ? |
//!   +-----+-----+-----+           +---+---+---+---+---+---+---+---+
//!            5     8
//! ```
//!
//! `let t = s;` copies those three words into `t` and marks `s` as dead. The
//! heap buffer does not move and is not copied. `s.clone()` allocates a second
//! buffer and copies the bytes. `&s` is one word: the address of the three
//! words above.
//!
//! The tests check `as_ptr()` before and after your function runs, so a
//! solution that clones will fail even when the text is right.
//!
//! Before you code, write answers in ANSWERS.md:
//! 1. After `let t = s;`, how many heap allocations exist? Who frees them?
//! 2. Why can `i32` be used after `let b = a;` but `String` cannot?
//! 3. In `take_last`, why can't you write `v[v.len() - 1]` to move the value out?

/// Returns whichever string is longer (in bytes); `a` wins a tie.
/// The other string is dropped here.
pub fn longer(a: String, b: String) -> String {
    todo!()
}

/// Upper-cases the ASCII letters of `s` in place and appends `!`.
pub fn shout(s: &mut String) {
    todo!()
}

/// Sum of the byte lengths of all words. Borrow only.
pub fn total_len(words: &[String]) -> usize {
    todo!()
}

/// Removes the last element and hands ownership of it to the caller.
pub fn take_last(v: &mut Vec<String>) -> Option<String> {
    todo!()
}

/// Exchanges the contents of two strings without allocating.
/// You cannot move out of a `&mut`, so look in `std::mem`.
pub fn swap_contents(a: &mut String, b: &mut String) {
    todo!()
}

/// Moves every string from `src` to the end of `dst`, keeping order.
/// `src` is left empty. No string buffer may be copied.
pub fn drain_into(src: &mut Vec<String>, dst: &mut Vec<String>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longer_picks_the_longer() {
        assert_eq!(longer("ab".into(), "abc".into()), "abc");
        assert_eq!(longer("abcd".into(), "abc".into()), "abcd");
    }

    #[test]
    fn longer_prefers_a_on_tie() {
        let a = String::from("left");
        let b = String::from("rght");
        let pa = a.as_ptr();
        let out = longer(a, b);
        assert_eq!(out.as_ptr(), pa);
    }

    #[test]
    fn longer_moves_instead_of_copying() {
        let a = String::from("hello world");
        let b = String::from("hi");
        let heap_addr = a.as_ptr();
        let out = longer(a, b);
        assert_eq!(out.as_ptr(), heap_addr, "the heap buffer must be reused");
    }

    #[test]
    fn shout_edits_in_place() {
        let mut s = String::from("low voice 1");
        shout(&mut s);
        assert_eq!(s, "LOW VOICE 1!");
    }

    #[test]
    fn total_len_sums_bytes() {
        let words = vec!["a".to_string(), "bcd".to_string(), String::new()];
        assert_eq!(total_len(&words), 4);
        assert_eq!(total_len(&[]), 0);
    }

    #[test]
    fn take_last_moves_out() {
        let mut v = vec!["x".to_string(), "last one".to_string()];
        let heap_addr = v[1].as_ptr();
        let got = take_last(&mut v).unwrap();
        assert_eq!(got.as_ptr(), heap_addr);
        assert_eq!(v, ["x"]);
        assert_eq!(take_last(&mut Vec::new()), None);
    }

    #[test]
    fn swap_contents_swaps_buffers() {
        let mut a = String::from("first");
        let mut b = String::from("second, longer");
        let (pa, pb) = (a.as_ptr(), b.as_ptr());
        swap_contents(&mut a, &mut b);
        assert_eq!((a.as_str(), b.as_str()), ("second, longer", "first"));
        assert_eq!((a.as_ptr(), b.as_ptr()), (pb, pa));
    }

    #[test]
    fn drain_into_moves_everything() {
        let mut src = vec!["c".to_string(), "d".to_string()];
        let mut dst = vec!["a".to_string(), "b".to_string()];
        let addrs: Vec<_> = src.iter().map(|s| s.as_ptr()).collect();
        drain_into(&mut src, &mut dst);
        assert!(src.is_empty());
        assert_eq!(dst, ["a", "b", "c", "d"]);
        assert_eq!(dst[2].as_ptr(), addrs[0]);
        assert_eq!(dst[3].as_ptr(), addrs[1]);
    }
}
