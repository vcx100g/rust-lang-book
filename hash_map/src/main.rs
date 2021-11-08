// hashmap is faster than array for access speed
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    dbg!(&scores);

    println!("{:?}", &scores.get("Blue"));

    // convert vector to hashmap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // _ placeholder type
    let mut scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    dbg!(scores2);

    // insert name and value separately
    let field_name = String::from("Color");
    let field_value = String::from("Blue");
    let mut color_map = HashMap::new();
    color_map.insert(field_name, field_value);
    dbg!(color_map);

    // get hash value
    let blue = String::from("Blue");
    dbg!(scores.get(&blue));

    // loop hashmap
    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }

    // update key value
    scores.insert(blue, 25);
    dbg!(&scores);

    // upsert key value, insert only if not exists
    scores.entry(String::from("Red")).or_insert(30);
    dbg!(&scores);

    let text = "hello world wonderful world";
    let mut text_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        // *count can update immutable variable because reference
        *count += 1
    }
    println!("{:?}", &text_map);
}
