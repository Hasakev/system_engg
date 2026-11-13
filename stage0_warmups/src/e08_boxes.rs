//! e08: `Box`, `Option` and `Drop`, by way of a linked stack.
//!
//! `Box<T>` is one pointer that owns one heap allocation. When the box goes
//! out of scope, the allocation is freed. A linked list is a chain of boxes:
//!
//! ```text
//!   stack                 heap
//!   +------+-----+        +-------+------+     +-------+------+     +-------+------+
//!   | head | len |   +--> | value | next | --> | value | next | --> | value | None |
//!   +------+-----+   |    +-------+------+     +-------+------+     +-------+------+
//!      |             |        3                    2                    1
//!      +-------------+
//! ```
//!
//! Each node owns the next one, so dropping the head frees the whole chain.
//!
//! The tools you need are `Option::take` (move the value out, leave `None`)
//! and `Option::as_ref` / `as_mut` (borrow what is inside).
//!
//! Last step, after everything else passes: the compiler-generated drop frees
//! a node by first dropping its `next` field, which drops that node's `next`,
//! and so on. That is one call-stack frame per node. Run
//!
//! ```text
//!   cargo test -p warmups e08 -- --ignored
//! ```
//!
//! and watch a real stack overflow. Then add `impl<T> Drop for Stack<T>` that
//! frees the nodes in a loop, and run it again.
//!
//! Before you code, write answers in ANSWERS.md:
//! 1. Why does `struct Node<T> { value: T, next: Option<Node<T>> }` not compile?
//! 2. `size_of::<Option<Box<Node<u8>>>>()` equals `size_of::<usize>()`. Where is the tag?

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Puts `value` on top.
    pub fn push(&mut self, value: T) {
        todo!()
    }

    /// Removes and returns the top value.
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    pub fn peek(&self) -> Option<&T> {
        todo!()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Reverses the list by re-pointing the existing nodes.
    /// No new allocations and no moving of values between nodes.
    pub fn reverse(&mut self) {
        todo!()
    }
}

// TODO (last step): `impl<T> Drop for Stack<T>` that frees nodes in a loop.

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn starts_empty() {
        let mut s = Stack::<i32>::new();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
        assert_eq!(s.peek(), None);
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn lifo_order() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.len(), 3);
        assert_eq!(s.peek(), Some(&3));
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
        assert!(s.is_empty());
    }

    #[test]
    fn peek_mut_edits_the_top() {
        let mut s = Stack::new();
        s.push(String::from("draft"));
        s.peek_mut().unwrap().push_str(" v2");
        assert_eq!(s.pop().as_deref(), Some("draft v2"));
    }

    #[test]
    fn reverse_relinks_without_copying() {
        let mut s = Stack::new();
        for word in ["a", "b", "c"] {
            s.push(word.to_string());
        }
        let top_buffer = s.peek().unwrap().as_ptr();
        s.reverse();
        assert_eq!(s.len(), 3);
        assert_eq!(s.pop().as_deref(), Some("a"));
        assert_eq!(s.pop().as_deref(), Some("b"));
        let last = s.pop().unwrap();
        assert_eq!(last, "c");
        assert_eq!(last.as_ptr(), top_buffer);
    }

    #[test]
    fn reverse_small_lists() {
        let mut s = Stack::<u8>::new();
        s.reverse();
        assert!(s.is_empty());
        s.push(9);
        s.reverse();
        assert_eq!(s.pop(), Some(9));
    }

    /// Counts how many times values of this type are dropped.
    struct Tracked(Rc<Cell<usize>>);
    impl Drop for Tracked {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    #[test]
    fn every_value_is_dropped_exactly_once() {
        let drops = Rc::new(Cell::new(0));
        {
            let mut s = Stack::new();
            for _ in 0..10 {
                s.push(Tracked(drops.clone()));
            }
            drop(s.pop());
            assert_eq!(drops.get(), 1);
        }
        assert_eq!(drops.get(), 10);
    }

    #[test]
    #[ignore = "overflows the call stack until you write an iterative Drop"]
    fn dropping_a_long_list_does_not_overflow_the_call_stack() {
        let mut s = Stack::new();
        for i in 0..2_000_000u32 {
            s.push(i);
        }
        drop(s);
    }
}
