// b4: the compiler cannot guess which input the output borrows from.
//
// Predict the error, then compile and read the whole message, including the
// `help:` section. Fix the signature. Then explain in ANSWERS.md why
// `fn first(a: &str) -> &str` needs no annotation but this function does.

fn pick<'direction>(a: &'direction str, b: &'direction str, use_a: bool) -> &'direction str {
    if use_a {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", pick("left", "right", false));
}
