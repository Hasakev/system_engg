//! Capstone A: the logic behind `wc`, `cat` and `grep`.
//!
//! The three binaries in `src/bin/` are thin wrappers that parse arguments,
//! open files and call these functions. Keeping the logic here, generic over
//! `Read` and `Write`, is what makes it testable without touching the disk.
//!
//! The lesson of this module is what `read` really promises:
//!
//! ```text
//!   you ask:   read(buf) with an 8192-byte buffer
//!   you get:   Ok(n) with any n from 1 to 8192     a "short read", normal
//!              Ok(0)                               end of input
//!              Err(kind = Interrupted)             a signal arrived, try again
//!              Err(anything else)                  a real failure
//! ```
//!
//! A file usually fills the buffer. A pipe, terminal or socket gives you
//! whatever has arrived so far. The tests feed your code one byte per call and
//! inject an `Interrupted`, so a word or line can straddle two reads.
//!
//! Write the read loops by hand. No `io::copy`, `read_to_end` or
//! `read_to_string` in this file.

use crate::e02_slices::find_subslice;
use std::io::{self, BufRead, Read, Write};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    /// Number of `\n` bytes.
    pub lines: usize,
    /// Number of maximal runs of non-whitespace bytes
    /// (`u8::is_ascii_whitespace` decides what whitespace is).
    pub words: usize,
    pub bytes: usize,
}

/// Counts lines, words and bytes, reading until end of input.
pub fn count<R: Read>(mut input: R) -> io::Result<Counts> {
    todo!()
}

/// Copies everything from `input` to `out` and returns the number of bytes
/// copied. Must not lose data on short reads.
pub fn copy<R: Read, W: Write>(mut input: R, out: &mut W) -> io::Result<u64> {
    todo!()
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GrepOpts {
    /// `-n`: prefix each printed line with its 1-based line number.
    pub line_numbers: bool,
    /// `-v`: print the lines that do not contain the pattern.
    pub invert: bool,
}

/// Prints the lines of `input` that contain `pattern` as a plain substring,
/// and returns how many lines were printed.
///
/// Output for each selected line is `[prefix:][lineno:]line\n`. A final line
/// with no trailing newline is still a line, and is printed with one.
/// Lines are bytes, not necessarily UTF-8: see `BufRead::read_until`.
/// Use your `find_subslice` for the matching.
pub fn grep<R: BufRead, W: Write>(
    pattern: &[u8],
    mut input: R,
    out: &mut W,
    prefix: Option<&str>,
    opts: GrepOpts,
) -> io::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    /// Delivers one byte per `read` call, the shortest legal short read.
    struct Trickle<'a>(&'a [u8]);
    impl Read for Trickle<'_> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            match (self.0.split_first(), buf.first_mut()) {
                (Some((&byte, rest)), Some(slot)) => {
                    *slot = byte;
                    self.0 = rest;
                    Ok(1)
                }
                _ => Ok(0),
            }
        }
    }

    /// Fails the first `read` with `Interrupted`, then behaves.
    struct Flaky<'a> {
        data: &'a [u8],
        tripped: bool,
    }
    impl Read for Flaky<'_> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if !self.tripped {
                self.tripped = true;
                return Err(io::Error::from(io::ErrorKind::Interrupted));
            }
            self.data.read(buf)
        }
    }

    struct Broken;
    impl Read for Broken {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("boom"))
        }
    }

    const TEXT: &[u8] = b"one two\n  three\tfour\n\nfive";

    #[test]
    fn count_basic() {
        let expected = Counts {
            lines: 3,
            words: 5,
            bytes: TEXT.len(),
        };
        assert_eq!(count(TEXT).unwrap(), expected);
        assert_eq!(count(&b""[..]).unwrap(), Counts::default());
    }

    #[test]
    fn count_is_the_same_across_short_reads() {
        assert_eq!(count(Trickle(TEXT)).unwrap(), count(TEXT).unwrap());
    }

    #[test]
    fn count_handles_big_input() {
        let big = b"word \n".repeat(20_000);
        let c = count(&big[..]).unwrap();
        assert_eq!((c.lines, c.words, c.bytes), (20_000, 20_000, 120_000));
    }

    #[test]
    fn count_retries_on_interrupted_and_reports_real_errors() {
        let flaky = Flaky {
            data: TEXT,
            tripped: false,
        };
        assert_eq!(count(flaky).unwrap(), count(TEXT).unwrap());
        assert!(count(Broken).is_err());
    }

    #[test]
    fn copy_moves_every_byte() {
        let data: Vec<u8> = (0..=255u8).cycle().take(50_000).collect();
        let mut out = Vec::new();
        assert_eq!(copy(&data[..], &mut out).unwrap(), 50_000);
        assert_eq!(out, data);
    }

    #[test]
    fn copy_survives_short_reads_and_interrupts() {
        let mut out = Vec::new();
        assert_eq!(copy(Trickle(TEXT), &mut out).unwrap(), TEXT.len() as u64);
        assert_eq!(out, TEXT);

        let mut out = Vec::new();
        copy(
            Flaky {
                data: TEXT,
                tripped: false,
            },
            &mut out,
        )
        .unwrap();
        assert_eq!(out, TEXT);

        assert!(copy(Broken, &mut Vec::new()).is_err());
    }

    fn run_grep(
        pattern: &str,
        text: &[u8],
        prefix: Option<&str>,
        opts: GrepOpts,
    ) -> (usize, String) {
        let mut out = Vec::new();
        let n = grep(pattern.as_bytes(), text, &mut out, prefix, opts).unwrap();
        (n, String::from_utf8(out).unwrap())
    }

    const LOG: &[u8] = b"boot ok\nerror: disk\nwarn: fan\nerror: net";

    #[test]
    fn grep_prints_matching_lines() {
        let (n, out) = run_grep("error", LOG, None, GrepOpts::default());
        assert_eq!(n, 2);
        assert_eq!(out, "error: disk\nerror: net\n");
    }

    #[test]
    fn grep_line_numbers() {
        let opts = GrepOpts {
            line_numbers: true,
            invert: false,
        };
        let (_, out) = run_grep("error", LOG, None, opts);
        assert_eq!(out, "2:error: disk\n4:error: net\n");
    }

    #[test]
    fn grep_invert() {
        let opts = GrepOpts {
            line_numbers: false,
            invert: true,
        };
        let (n, out) = run_grep("error", LOG, None, opts);
        assert_eq!(n, 2);
        assert_eq!(out, "boot ok\nwarn: fan\n");
    }

    #[test]
    fn grep_prefix_then_line_number() {
        let opts = GrepOpts {
            line_numbers: true,
            invert: false,
        };
        let (_, out) = run_grep("fan", LOG, Some("sys.log"), opts);
        assert_eq!(out, "sys.log:3:warn: fan\n");
    }

    #[test]
    fn grep_no_match_and_empty_input() {
        assert_eq!(
            run_grep("zzz", LOG, None, GrepOpts::default()),
            (0, String::new())
        );
        assert_eq!(
            run_grep("a", b"", None, GrepOpts::default()),
            (0, String::new())
        );
    }

    #[test]
    fn grep_works_when_lines_straddle_reads() {
        let reader = BufReader::with_capacity(3, Trickle(LOG));
        let mut out = Vec::new();
        let n = grep(b"error", reader, &mut out, None, GrepOpts::default()).unwrap();
        assert_eq!(n, 2);
        assert_eq!(out, b"error: disk\nerror: net\n");
    }

    #[test]
    fn grep_passes_non_utf8_bytes_through() {
        let text = b"ok \xFF\xFE match\nskip\n";
        let mut out = Vec::new();
        grep(b"match", &text[..], &mut out, None, GrepOpts::default()).unwrap();
        assert_eq!(out, b"ok \xFF\xFE match\n");
    }
}
