// b5: iterator invalidation.
//
// Python lets you append to a list while looping over it, with surprising
// results. Predict what Rust says, then compile.
//
// Fix it so that every even number is followed by its double:
// [1, 2, 3, 4] becomes [1, 2, 4, 3, 4, 8]. Build a new Vec.

fn main() {
    let mut v = vec![1, 2, 3, 4];
    let mut new_v = vec![];
    for x in &v {
        new_v.push(*x);
        if x % 2 == 0 {
            new_v.push(*x * 2);
        }
    }
    v = new_v; // re allocate memory address, auto free original v
               // new_v also moves out
    println!("{v:?}");
}
