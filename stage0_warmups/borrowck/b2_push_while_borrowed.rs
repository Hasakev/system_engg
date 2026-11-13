// b2: mutation while a shared borrow is alive.
//
// Predict the error, then compile. Before fixing it, answer in ANSWERS.md:
// the same code in C compiles and sometimes prints garbage. Draw the heap
// before and after `push` and explain why. (Hint: capacity 3, length 3.)
//
// Fix it so it prints the ORIGINAL first element, 1, and v ends as [1, 2, 3, 4].

fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("first = {first}, v = {v:?}");
}
