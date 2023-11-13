pub fn run() {
    let mut s = String::from("Hello");
    change(&mut s); // There can only be one mutable reference at a time
    let len = calculate_length(&s);
    println!("The length of {} is {}", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
