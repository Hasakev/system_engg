// b1: use after move.
//
// Predict the compiler's complaint and the line it points at, then compile.
// Fix it WITHOUT `.clone()`: change what `print_len` asks for.

fn print_len(s: String) {
    println!("{} bytes", s.len());
}

fn main() {
    let s = String::from("hello");
    print_len(s);
    println!("{s}");
}
