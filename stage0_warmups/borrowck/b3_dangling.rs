// b3: returning a reference to a local.
//
// Predict the error, then compile. Where does `upper` live, and when is it
// freed? Fix it by changing the return type so the caller owns the result.

fn first_line_upper(text: &str) -> &str {
    let upper = text.to_uppercase();
    upper.lines().next().unwrap_or("")
}

fn main() {
    println!("{}", first_line_upper("hello\nworld"));
}
