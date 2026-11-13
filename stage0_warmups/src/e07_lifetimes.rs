//! e07: lifetimes.
//!
//! A lifetime names a region of code during which a reference is valid. The
//! annotation `'a` never changes how long anything lives; it tells the
//! compiler which input a returned reference points into, so it can reject
//! callers that would keep the result past the data.
//!
//! ```text
//!   fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
//!
//!   let a = String::from("long lived");
//!   let r;
//!   {
//!       let b = String::from("short");
//!       r = longest(&a, &b);      // result may point into b ...
//!   }                             // ... and b is freed here
//!   println!("{r}");              // so this line is rejected
//! ```
//!
//! A struct that holds a reference needs the same annotation: `Words<'a>`
//! cannot outlive the text it is cutting up.
//!
//! Everything here is zero-copy. The tests check that what you return points
//! inside the input. `Words` grows into your shell's tokenizer.
//!
//! Before you code, write answers in ANSWERS.md:
//! 1. `fn first_word(s: &str) -> &str` in e02 has no annotations. Why was it accepted?
//! 2. In `Words<'a>`, why is the iterator's item `&'a str` and not a
//!    reference tied to the `&mut self` borrow in `next`?

/// The longer of the two (in bytes); `a` wins a tie.
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!()
}

/// Splits at the first `sep`: `("key", "value")` for `"key=value"` and `'='`.
/// Write it yourself; no `str::split_once`.
pub fn split_once_char(s: &str, sep: char) -> Option<(&str, &str)> {
    todo!()
}

/// Iterator over the whitespace-separated words of a string.
/// Write it yourself; no `split_whitespace`.
pub struct Words<'a> {
    /// The part of the input not yet consumed.
    rest: &'a str,
}

impl<'a> Words<'a> {
    pub fn new(input: &'a str) -> Self {
        todo!()
    }
}

impl<'a> Iterator for Words<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inside(outer: &str, inner: &str) -> bool {
        let (o, i) = (outer.as_ptr() as usize, inner.as_ptr() as usize);
        i >= o && i + inner.len() <= o + outer.len()
    }

    #[test]
    fn longest_picks_by_length() {
        assert_eq!(longest("abc", "de"), "abc");
        assert_eq!(longest("a", "de"), "de");
        let (a, b) = (String::from("same"), String::from("size"));
        assert_eq!(longest(&a, &b).as_ptr(), a.as_ptr());
    }

    #[test]
    fn split_once_char_cases() {
        assert_eq!(split_once_char("key=value", '='), Some(("key", "value")));
        assert_eq!(split_once_char("a=b=c", '='), Some(("a", "b=c")));
        assert_eq!(split_once_char("=v", '='), Some(("", "v")));
        assert_eq!(split_once_char("k=", '='), Some(("k", "")));
        assert_eq!(split_once_char("none", '='), None);
    }

    #[test]
    fn split_once_char_handles_multibyte_separator() {
        assert_eq!(split_once_char("left→right", '→'), Some(("left", "right")));
    }

    #[test]
    fn words_splits_on_any_whitespace() {
        let got: Vec<_> = Words::new("  ls -la\t/tmp \n").collect();
        assert_eq!(got, ["ls", "-la", "/tmp"]);
    }

    #[test]
    fn words_edge_cases() {
        assert_eq!(Words::new("").count(), 0);
        assert_eq!(Words::new("   ").count(), 0);
        assert_eq!(Words::new("one").collect::<Vec<_>>(), ["one"]);
        let mut w = Words::new("a");
        assert_eq!(w.next(), Some("a"));
        assert_eq!(w.next(), None);
        assert_eq!(w.next(), None);
    }

    #[test]
    fn words_are_windows_into_the_input() {
        let input = String::from("echo hello world");
        let words: Vec<&str> = Words::new(&input).collect();
        // The iterator is gone; the words are still valid because they
        // borrow from `input`, not from the iterator.
        assert!(words.iter().all(|w| inside(&input, w)));
        assert_eq!(words[1].as_ptr(), input[5..].as_ptr());
    }
}
