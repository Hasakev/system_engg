//! e06: generics and const generics, by way of a ring buffer.
//!
//! A ring buffer is a fixed array used as a queue. When it is full, a new
//! element overwrites the oldest one. Kernels use them for log buffers, audio
//! drivers for samples, and you will use this one for shell history and for
//! the visualiser's event log.
//!
//! ```text
//!   N = 4, after pushing a, b, c:
//!
//!     slots: [ a | b | c | . ]      head = 0 (oldest), len = 3
//!              ^head
//!
//!   push d, then push e. The buffer was full, so e overwrites a:
//!
//!     slots: [ e | b | c | d ]      head = 1, len = 4
//!                  ^head            logical order: b, c, d, e
//!
//!   logical index i lives in slot (head + i) % N
//! ```
//!
//! `RingBuffer<T, const N: usize>` is compiled separately for each `T` and
//! `N` you use. That is monomorphisation: zero runtime cost, more machine code.
//!
//! Before you code, write answers in ANSWERS.md:
//! 1. Where do the `N` slots live when you write `let rb = RingBuffer::<u64, 8>::new();`?
//!    How many heap allocations does that line perform?
//! 2. Why are the slots `Option<T>` and not plain `T`?

pub struct RingBuffer<T, const N: usize> {
    slots: [Option<T>; N],
    /// Slot index of the oldest element.
    head: usize,
    len: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    /// An empty buffer. `N` must be at least 1.
    /// Hint: `[None; N]` needs `T: Copy`. See `std::array::from_fn`.
    pub fn new() -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn is_full(&self) -> bool {
        todo!()
    }

    /// Appends `value` as the newest element. If the buffer was full, the
    /// oldest element is evicted and returned.
    pub fn push(&mut self, value: T) -> Option<T> {
        todo!()
    }

    /// Removes and returns the oldest element.
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    /// The element at logical index `i`, where 0 is the oldest.
    pub fn get(&self, i: usize) -> Option<&T> {
        todo!()
    }

    /// Oldest to newest. This one is written for you; `Iter::next` is not.
    pub fn iter(&self) -> Iter<'_, T, N> {
        Iter { buf: self, next: 0 }
    }
}

/// Borrowing iterator over a `RingBuffer`. The `'a` says "this iterator must
/// not outlive the buffer it reads from"; e07 covers that properly.
pub struct Iter<'a, T, const N: usize> {
    buf: &'a RingBuffer<T, N>,
    /// Logical index of the element `next` will yield.
    next: usize,
}

impl<'a, T, const N: usize> Iterator for Iter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_empty() {
        let rb = RingBuffer::<i32, 4>::new();
        assert_eq!(rb.len(), 0);
        assert!(rb.is_empty());
        assert!(!rb.is_full());
        assert_eq!(rb.get(0), None);
    }

    #[test]
    fn fifo_order() {
        let mut rb = RingBuffer::<i32, 4>::new();
        assert_eq!(rb.push(1), None);
        assert_eq!(rb.push(2), None);
        assert_eq!(rb.pop(), Some(1));
        assert_eq!(rb.pop(), Some(2));
        assert_eq!(rb.pop(), None);
        assert!(rb.is_empty());
    }

    #[test]
    fn overwrites_oldest_when_full() {
        let mut rb = RingBuffer::<char, 4>::new();
        for c in ['a', 'b', 'c', 'd'] {
            assert_eq!(rb.push(c), None);
        }
        assert!(rb.is_full());
        assert_eq!(rb.push('e'), Some('a'));
        assert_eq!(rb.len(), 4);
        assert_eq!(rb.iter().copied().collect::<String>(), "bcde");
        assert_eq!(rb.get(0), Some(&'b'));
        assert_eq!(rb.get(3), Some(&'e'));
        assert_eq!(rb.get(4), None);
    }

    #[test]
    fn wraps_around_many_times() {
        let mut rb = RingBuffer::<usize, 3>::new();
        for i in 0..100 {
            rb.push(i);
            if i % 4 == 0 {
                rb.pop();
            }
        }
        let got: Vec<_> = rb.iter().copied().collect();
        assert_eq!(got, [97, 98, 99]);
    }

    #[test]
    fn capacity_one() {
        let mut rb = RingBuffer::<&str, 1>::new();
        assert_eq!(rb.push("x"), None);
        assert_eq!(rb.push("y"), Some("x"));
        assert_eq!(rb.pop(), Some("y"));
        assert_eq!(rb.pop(), None);
    }

    #[test]
    fn works_with_owned_non_copy_values() {
        let mut rb = RingBuffer::<String, 2>::new();
        rb.push("one".to_string());
        rb.push("two".to_string());
        assert_eq!(rb.push("three".to_string()).as_deref(), Some("one"));
        assert_eq!(rb.pop().as_deref(), Some("two"));
        assert_eq!(rb.get(0).map(String::as_str), Some("three"));
    }
}
