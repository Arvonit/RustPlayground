pub fn run() {
    // After `takes_ownership` is called, `s` is no longer valid because its value is moved to
    // `some_string` in `takes_ownership()`.
    let s = String::from("Arvind");
    takes_ownership(s);

    // After `makes_copy` is called, `x` is still valid as its value is just copied to
    // `some_integer`.
    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    // `drop()` is called on `some_string` once the function exits, freeing its underlying memory.
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
