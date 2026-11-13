# Stage 0: warm-ups

Goal: get Rust's ownership model and the basic machine-level vocabulary into
your hands, so later stages are about systems and not about fighting syntax.

You know Python, `malloc` and the stack, so this stage moves quickly and leans
on what Rust does differently: who owns each allocation, and who may touch it.

Everything uses the standard library only. Expect one to two weeks of evenings.

## Order of work

| # | Where | Topic | Tests |
|---|---|---|---|
| 1 | `borrowck/b1` to `b6` | Six programs the borrow checker rejects. Predict, compile, fix. | they compile and print the right thing |
| 2 | `src/e01_ownership.rs` | Moves, borrows, `mem::swap` | 8 |
| 3 | `src/e02_slices.rs` | Fat pointers, windows onto memory | 7 |
| 4 | `src/e03_bits.rs` | Shifts, masks, endianness, alignment | 6 |
| 5 | `src/e04_enums.rs` | Tagged unions and `match`: a stack machine | 9 |
| 6 | `src/e05_errors.rs` | `Result`, `?`, `From`, custom error types | 7 |
| 7 | `src/e06_generics.rs` | Const generics: a ring buffer | 6 |
| 8 | `src/e07_lifetimes.rs` | Zero-copy parsing | 6 |
| 9 | `src/e08_boxes.rs` | `Box`, `Option::take`, `Drop`: a linked stack | 6 + 1 ignored |
| 10 | `src/e09_unsafe.rs` | Raw pointers, a bump allocator | 9 |
| 11 | `src/tools.rs`, `src/bin/{wc,cat,grep}.rs` | Capstone A: real command-line tools | 13 + 15 end-to-end |
| 12 | `src/progress.rs`, `src/bin/progress.rs` | Capstone B: a tool that runs your tests and reports progress | 4 |

Each `src` file starts with a diagram and questions. Answer the questions in
[ANSWERS.md](ANSWERS.md) before you write the code.

## Commands

```sh
# the compile-fix puzzles (from this folder)
rustc --edition 2021 borrowck/b1_use_after_move.rs --out-dir borrowck/out && ./borrowck/out/b1_use_after_move

# one module, one test, everything
cargo test -p warmups e01
cargo test -p warmups e01_ownership::tests::longer_moves_instead_of_copying
cargo test -p warmups --no-fail-fast

# show println! output from passing tests
cargo test -p warmups e04 -- --nocapture

# the end-to-end tests for the binaries
cargo test -p warmups --test cli

# run your tools
cargo run -p warmups --bin wc -- Cargo.toml
echo "hello" | cargo run -q -p warmups --bin grep -- -n ell

# lints and formatting, once a module passes
cargo clippy -p warmups
cargo fmt
```

On Windows, the puzzle binary is `borrowck\out\b1_use_after_move.exe`.

## Rules for yourself

- A test going green is the minimum. Many functions carry a constraint in
  their doc comment ("no `.clone()`", "no `.reverse()`", "write the loop by
  hand"). Those constraints are the exercise.
- When the compiler rejects something, write down in one sentence what bug it
  just prevented. If you cannot, ask.
- Every `unsafe` block gets a `// SAFETY:` comment explaining why it is sound.
- Stuck for more than 30 minutes: ask for a hint, not a solution.

## Done when

- `cargo test -p warmups --no-fail-fast` is green, including
  `cargo test -p warmups e08 -- --ignored`.
- `cargo clippy -p warmups` is quiet with the `#![allow(...)]` line in
  `src/lib.rs` deleted.
- `cargo run -p warmups --bin progress` shows every row full, using the tool
  you wrote.
- `ANSWERS.md` is filled in and reviewed.
