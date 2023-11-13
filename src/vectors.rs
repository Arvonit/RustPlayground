#[allow(unused)]
pub fn run() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3]; // We must make the vector mutable to add or remove elements
    let mut v = Vec::new();

    // Since we are pushing integers to the vector, the compiler implies `v` is a vector of i32
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading elements of a vector
    let v = vec![1, 2, 3, 4, 5];

    // If the vector doesn't have a third element, the program will panic
    let third = &v[2];
    println!("The third element is {}", third);

    // This is a safer way to handle index out of bounds errors
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Iterating over a vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // println!("{}", i);
        *i += 50; // Add 50 to each value in v
    }
}
