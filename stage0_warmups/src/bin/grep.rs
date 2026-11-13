//! `grep [-n] [-v] PATTERN [FILE...]`: print lines containing PATTERN.
//!
//! - Flags come before the pattern. `-n` adds line numbers, `-v` inverts.
//! - No files: search stdin.
//! - More than one file: prefix each printed line with `NAME:`.
//! - Exit status: 0 if any line was printed, 1 if none, 2 on any error
//!   (missing pattern, unknown flag, unreadable file). An unreadable file
//!   prints `grep: NAME: REASON` to stderr and the search continues.
//!
//! All matching goes through `warmups::tools::grep`.

use std::process::ExitCode;

fn main() -> ExitCode {
    todo!()
}
