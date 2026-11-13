//! `progress`: run this crate's tests and print one row per module.
//!
//! 1. Spawn `cargo test -p warmups --lib --no-fail-fast` with
//!    `std::process::Command` and capture its stdout. When this binary is
//!    started by `cargo run`, the `CARGO` environment variable holds the path
//!    to cargo; fall back to plain `cargo` if it is unset.
//! 2. Feed the captured lines to `warmups::progress::summarize`.
//! 3. Print one row per module, for example:
//!
//! ```text
//!   e01_ownership  [########]  8/8
//!   e02_slices     [###.....]  3/8
//!   e08_boxes      [........]  0/7  (1 ignored)
//! ```
//!
//! The exact layout is yours. Exit 0 when nothing failed, 1 otherwise.
//!
//! Questions to answer in ANSWERS.md once it works:
//! 1. Where did the child's stderr go when you captured only stdout?
//! 2. `Command::output` waits for the child to finish. What would you use to
//!    print rows while the tests are still running?

use std::process::ExitCode;

fn main() -> ExitCode {
    todo!()
}
