//! e05: errors as values.
//!
//! Rust has no exceptions. A function that can fail returns `Result<T, E>`,
//! and `?` means "on `Err`, convert the error with `From` and return early".
//!
//! ```text
//!   let n = text.parse::<i64>()?;
//!
//!   is shorthand for
//!
//!   let n = match text.parse::<i64>() {
//!       Ok(n) => n,
//!       Err(e) => return Err(From::from(e)),
//!   };
//! ```
//!
//! You will parse a small config format and report exactly what went wrong
//! and on which line:
//!
//! ```text
//!   # comment lines and blank lines are skipped
//!   threads = 4
//!   retries=3
//! ```
//!
//! Rules: trim whitespace around the line, the key and the value. Split at the
//! first `=`. Values are `i64`. Line numbers start at 1 and count every line,
//! skipped ones included.
//!
//! Before you code, write answers in ANSWERS.md:
//! 1. Which trait impl lets `?` turn an `io::Error` into a `ConfigError`?
//! 2. What is `Error::source` for, and which variants here should return `Some`?

use std::fmt;
use std::io::{self, BufRead};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ConfigError {
    /// A non-comment line had no `=`.
    MissingEquals { line: usize },
    /// Nothing but whitespace before the `=`.
    EmptyKey { line: usize },
    /// The value was not an `i64`.
    BadNumber { line: usize, source: ParseIntError },
    /// The reader failed.
    Io(io::Error),
}

impl fmt::Display for ConfigError {
    /// A one-line human message. It must include the line number when there is one.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        todo!()
    }
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        todo!()
    }
}

/// Parses the whole input, stopping at the first error.
pub fn parse_config<R: BufRead>(input: R) -> Result<Vec<(String, i64)>, ConfigError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::io::{BufReader, Read};

    #[test]
    fn parses_pairs_and_skips_noise() {
        let text = "# settings\n\nthreads = 4\n  retries=3  \nneg = -12\n";
        let got = parse_config(text.as_bytes()).unwrap();
        assert_eq!(
            got,
            [
                ("threads".to_string(), 4),
                ("retries".to_string(), 3),
                ("neg".to_string(), -12)
            ]
        );
    }

    #[test]
    fn empty_input_is_fine() {
        assert!(parse_config("".as_bytes()).unwrap().is_empty());
    }

    #[test]
    fn missing_equals_names_the_line() {
        let err = parse_config("a = 1\n# c\nbroken\n".as_bytes()).unwrap_err();
        assert!(
            matches!(err, ConfigError::MissingEquals { line: 3 }),
            "{err:?}"
        );
        assert!(err.to_string().contains('3'));
        assert!(err.source().is_none());
    }

    #[test]
    fn empty_key_names_the_line() {
        let err = parse_config("  = 5\n".as_bytes()).unwrap_err();
        assert!(matches!(err, ConfigError::EmptyKey { line: 1 }), "{err:?}");
    }

    #[test]
    fn bad_number_keeps_the_cause() {
        let err = parse_config("a = 1\nb = twelve\n".as_bytes()).unwrap_err();
        assert!(
            matches!(err, ConfigError::BadNumber { line: 2, .. }),
            "{err:?}"
        );
        assert!(err.to_string().contains('2'));
        let cause = err.source().expect("BadNumber should expose its cause");
        assert!(cause.downcast_ref::<ParseIntError>().is_some());
    }

    #[test]
    fn splits_at_the_first_equals_only() {
        let err = parse_config("a = 1 = 2\n".as_bytes()).unwrap_err();
        assert!(
            matches!(err, ConfigError::BadNumber { line: 1, .. }),
            "{err:?}"
        );
    }

    struct Broken;
    impl Read for Broken {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("disk on fire"))
        }
    }

    #[test]
    fn io_errors_pass_through() {
        let err = parse_config(BufReader::new(Broken)).unwrap_err();
        assert!(matches!(err, ConfigError::Io(_)), "{err:?}");
        assert!(err.source().is_some());
    }
}
