//! `cat [FILE...]`: copy files to stdout, in order.
//!
//! - No arguments: copy stdin to stdout.
//! - Bytes pass through untouched; the input need not be UTF-8.
//! - A file that cannot be read: print `cat: NAME: REASON` to stderr, carry on
//!   with the rest, and exit with status 1 at the end.
//!
//! All copying goes through `warmups::tools::copy`. Lock stdout once instead
//! of once per write, and ask yourself what `println!` would cost here.

use std::process::ExitCode;

fn main() -> ExitCode {
    todo!()
}
