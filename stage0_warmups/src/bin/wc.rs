//! `wc [FILE...]`: count lines, words and bytes.
//!
//! - No arguments: read stdin and print `LINES WORDS BYTES`.
//! - Per file: print `LINES WORDS BYTES NAME`.
//! - More than one file: finish with `LINES WORDS BYTES total`.
//! - Fields are separated by one space. No padding.
//! - A file that cannot be read: print `wc: NAME: REASON` to stderr, carry on
//!   with the rest, and exit with status 1 at the end.
//!
//! All counting goes through `warmups::tools::count`.

use std::process::ExitCode;

fn main() -> ExitCode {
    todo!()
}
