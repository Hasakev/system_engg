//! Capstone B: the tool that tracks your own progress.
//!
//! `cargo run -p warmups --bin progress` should run this crate's tests as a
//! child process, read what the test harness prints, and show one row per
//! module. You build the tool that reports on the rest of your work, and you
//! meet processes and pipes, the subject of the shell stage, for the first time.
//!
//! ```text
//!   progress (parent)                          cargo test (child)
//!   +-----------------+   spawn, then wait    +--------------------+
//!   | Command::new    | --------------------> | runs the tests     |
//!   | .output()       | <---- stdout pipe --- | prints result lines|
//!   +-----------------+                       +--------------------+
//! ```
//!
//! The harness prints one line per test:
//!
//! ```text
//!   test e01_ownership::tests::longer_picks_the_longer ... ok
//!   test e08_boxes::tests::lifo_order ... FAILED
//!   test e08_boxes::tests::long_list ... ignored, overflows the call stack
//! ```
//!
//! and other lines you must skip, such as `running 12 tests`, blank lines,
//! panic messages, and `test result: FAILED. 3 passed; 9 failed; ...`.
//!
//! This file holds the parsing, which is testable. `src/bin/progress.rs`
//! holds the process handling.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Passed,
    Failed,
    Ignored,
}

/// One parsed result line. Borrows from the line; allocates nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TestLine<'a> {
    /// First path segment, e.g. `e01_ownership`.
    pub module: &'a str,
    /// Last path segment, e.g. `longer_picks_the_longer`.
    pub name: &'a str,
    pub status: Status,
}

/// Parses one line of harness output. Returns `None` for anything that is
/// not a single test's result.
///
/// The status is the text after the last ` ... `: exactly `ok`, exactly
/// `FAILED`, or anything starting with `ignored`. A test path with no `::`
/// is not one of ours; return `None`.
pub fn parse_test_line(line: &str) -> Option<TestLine<'_>> {
    todo!()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleSummary {
    pub module: String,
    pub passed: usize,
    pub failed: usize,
    pub ignored: usize,
}

/// Tallies results per module, sorted by module name.
/// Lines that `parse_test_line` rejects are skipped.
pub fn summarize<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<ModuleSummary> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_each_status() {
        assert_eq!(
            parse_test_line("test e01_ownership::tests::longer_picks_the_longer ... ok"),
            Some(TestLine {
                module: "e01_ownership",
                name: "longer_picks_the_longer",
                status: Status::Passed
            })
        );
        assert_eq!(
            parse_test_line("test e08_boxes::tests::lifo_order ... FAILED").map(|t| t.status),
            Some(Status::Failed)
        );
        assert_eq!(
            parse_test_line("test e08_boxes::tests::long ... ignored, overflows the call stack")
                .map(|t| (t.module, t.name, t.status)),
            Some(("e08_boxes", "long", Status::Ignored))
        );
        assert_eq!(
            parse_test_line("test a::b ... ignored").map(|t| t.status),
            Some(Status::Ignored)
        );
    }

    #[test]
    fn skips_everything_else() {
        for line in [
            "",
            "running 12 tests",
            "test result: FAILED. 3 passed; 9 failed; 1 ignored; 0 measured; 0 filtered out",
            "test result: ok. 3 passed; 0 failed",
            "thread 'e01_ownership::tests::x' panicked at src/e01_ownership.rs:40:5:",
            "not yet implemented",
            "failures:",
            "    e01_ownership::tests::longer_picks_the_longer",
            "test top_level_no_module ... ok",
            "test a::b ... maybe",
        ] {
            assert_eq!(parse_test_line(line), None, "{line:?}");
        }
    }

    #[test]
    fn parsed_fields_borrow_from_the_line() {
        let line = String::from("test m::tests::t ... ok");
        let parsed = parse_test_line(&line).unwrap();
        assert_eq!(parsed.module.as_ptr(), line[5..].as_ptr());
    }

    #[test]
    fn summarize_tallies_and_sorts() {
        let output = "\
running 6 tests
test e02_slices::tests::a ... ok
test e01_ownership::tests::a ... FAILED
test e02_slices::tests::b ... FAILED
test e01_ownership::tests::b ... ok
test e02_slices::tests::c ... ok
test e08_boxes::tests::long ... ignored, reason

test result: FAILED. 3 passed; 2 failed; 1 ignored; 0 measured; 0 filtered out
";
        let summary = summarize(output.lines());
        let rows: Vec<_> = summary
            .iter()
            .map(|m| (m.module.as_str(), m.passed, m.failed, m.ignored))
            .collect();
        assert_eq!(
            rows,
            [
                ("e01_ownership", 1, 1, 0),
                ("e02_slices", 2, 1, 0),
                ("e08_boxes", 0, 0, 1)
            ]
        );
        assert!(summarize("nothing here\n".lines()).is_empty());
    }
}
