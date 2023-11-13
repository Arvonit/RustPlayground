pub fn run() {
    let mut text = String::from("Hello, world!");
    let word = first_word(&text);
    println!("The first word is: {}.", word);
    text.clear(); // We have to clear after using `word` because we cannot have an immutable
                  // reference in scope while mutating `text`
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
