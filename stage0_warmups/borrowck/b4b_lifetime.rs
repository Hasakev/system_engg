fn pick<'direction>(a: &'direction str, b: &'direction str, flag: bool) -> &'direction str {
    if flag {
        a
    } else {
        b
    }
}

fn main() {
    let a = String::from("left");
    let b = String::from("right");
    let r = pick(&a, &b, true);
    drop(b); // b is dropped here, static analyser gets cooked, because r is still referencing b, but b is no longer valid

    println!("r: {}", r);
}
