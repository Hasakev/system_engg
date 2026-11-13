//! End-to-end tests for the `wc`, `cat` and `grep` binaries.
//!
//! Each test starts your compiled program as a child process, exactly as a
//! shell would, and checks its stdout, stderr and exit status.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

/// A file in the temp directory that is deleted when dropped.
struct TempFile(PathBuf);

impl TempFile {
    fn new(tag: &str, contents: &[u8]) -> Self {
        let path = std::env::temp_dir().join(format!("warmups-{}-{tag}", std::process::id()));
        std::fs::write(&path, contents).unwrap();
        TempFile(path)
    }

    fn path(&self) -> &str {
        self.0.to_str().unwrap()
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn run(bin: &str, args: &[&str], stdin: &[u8]) -> Output {
    let mut child = Command::new(bin)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    // The child may exit without reading stdin; a broken pipe here is fine.
    let _ = child.stdin.take().unwrap().write_all(stdin);
    child.wait_with_output().unwrap()
}

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

const WC: &str = env!("CARGO_BIN_EXE_wc");
const CAT: &str = env!("CARGO_BIN_EXE_cat");
const GREP: &str = env!("CARGO_BIN_EXE_grep");
const MISSING: &str = "/definitely/not/a/real/file";

#[test]
fn wc_reads_stdin() {
    let out = run(WC, &[], b"one two\nthree\n");
    assert_eq!(stdout(&out), "2 3 14\n");
    assert_eq!(out.status.code(), Some(0));
}

#[test]
fn wc_one_file() {
    let f = TempFile::new("wc-one", b"one two\nthree\n");
    let out = run(WC, &[f.path()], b"");
    assert_eq!(stdout(&out), format!("2 3 14 {}\n", f.path()));
    assert_eq!(out.status.code(), Some(0));
}

#[test]
fn wc_many_files_prints_total() {
    let a = TempFile::new("wc-a", b"a b c\n");
    let b = TempFile::new("wc-b", b"d\ne\n");
    let out = run(WC, &[a.path(), b.path()], b"");
    let expected = format!("1 3 6 {}\n2 2 4 {}\n3 5 10 total\n", a.path(), b.path());
    assert_eq!(stdout(&out), expected);
}

#[test]
fn wc_missing_file_reports_and_continues() {
    let a = TempFile::new("wc-ok", b"x\n");
    let out = run(WC, &[MISSING, a.path()], b"");
    assert_eq!(out.status.code(), Some(1));
    assert!(stdout(&out).contains(&format!("1 1 2 {}\n", a.path())));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.starts_with("wc: "), "stderr was: {stderr}");
    assert!(stderr.contains(MISSING));
}

#[test]
fn cat_copies_stdin() {
    let out = run(CAT, &[], b"through\nthe pipe");
    assert_eq!(out.stdout, b"through\nthe pipe");
    assert_eq!(out.status.code(), Some(0));
}

#[test]
fn cat_concatenates_files_in_order() {
    let a = TempFile::new("cat-a", b"first\n");
    let b = TempFile::new("cat-b", b"second\n");
    let out = run(CAT, &[a.path(), b.path(), a.path()], b"");
    assert_eq!(out.stdout, b"first\nsecond\nfirst\n");
}

#[test]
fn cat_is_binary_safe() {
    let bytes: Vec<u8> = (0..=255u8).cycle().take(100_000).collect();
    let f = TempFile::new("cat-bin", &bytes);
    let out = run(CAT, &[f.path()], b"");
    assert_eq!(out.stdout, bytes);
}

#[test]
fn cat_missing_file_reports_and_continues() {
    let a = TempFile::new("cat-ok", b"still here\n");
    let out = run(CAT, &[MISSING, a.path()], b"");
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(out.stdout, b"still here\n");
    assert!(String::from_utf8_lossy(&out.stderr).starts_with("cat: "));
}

const LOG: &[u8] = b"boot ok\nerror: disk\nwarn: fan\nerror: net\n";

#[test]
fn grep_searches_stdin() {
    let out = run(GREP, &["error"], LOG);
    assert_eq!(stdout(&out), "error: disk\nerror: net\n");
    assert_eq!(out.status.code(), Some(0));
}

#[test]
fn grep_flags() {
    assert_eq!(
        stdout(&run(GREP, &["-n", "error"], LOG)),
        "2:error: disk\n4:error: net\n"
    );
    assert_eq!(
        stdout(&run(GREP, &["-v", "error"], LOG)),
        "boot ok\nwarn: fan\n"
    );
    assert_eq!(stdout(&run(GREP, &["-n", "-v", "o"], LOG)), "3:warn: fan\n");
}

#[test]
fn grep_exit_status_one_when_nothing_matches() {
    let out = run(GREP, &["zzz"], LOG);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
}

#[test]
fn grep_one_file_has_no_prefix() {
    let f = TempFile::new("grep-one", LOG);
    assert_eq!(stdout(&run(GREP, &["fan", f.path()], b"")), "warn: fan\n");
}

#[test]
fn grep_many_files_are_prefixed() {
    let a = TempFile::new("grep-a", b"apple\nbanana\n");
    let b = TempFile::new("grep-b", b"cherry\napple pie\n");
    let out = run(GREP, &["-n", "apple", a.path(), b.path()], b"");
    let expected = format!("{}:1:apple\n{}:2:apple pie\n", a.path(), b.path());
    assert_eq!(stdout(&out), expected);
}

#[test]
fn grep_usage_errors_exit_two() {
    assert_eq!(run(GREP, &[], b"").status.code(), Some(2));
    assert_eq!(run(GREP, &["-x", "pat"], b"").status.code(), Some(2));
}

#[test]
fn grep_missing_file_exits_two_but_still_searches() {
    let a = TempFile::new("grep-ok", b"needle\n");
    let out = run(GREP, &["needle", MISSING, a.path()], b"");
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(stdout(&out), format!("{}:needle\n", a.path()));
    assert!(String::from_utf8_lossy(&out.stderr).starts_with("grep: "));
}
