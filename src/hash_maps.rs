use std::collections::HashMap;

#[allow(unused)]
pub fn run() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Another way of constructing a hash map, not quite elegant though
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let name = String::from("Favorite color");
    let value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(name, value);
    // name and value are invalid at this point

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing a value in the hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    if let Some(score) = score {
        println!("The score for team {} is {}.", team_name, score);
    }

    // Iterating over a hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // We can update a hash map's value by overwriting it with an insert
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // We can delete a key, value pair
    scores.remove(&String::from("Yellow"));
    println!("{:?}", scores);

    // We can only insert a value if the key doesn't exist
    scores.entry(String::from("Yellow")).or_insert(40);
    scores.entry(String::from("Blue")).or_insert(40);
    println!("{:?}", scores);

    // Update entry's value based on old value
    for (key, value) in &mut scores {
        *value += 10;
    }
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
