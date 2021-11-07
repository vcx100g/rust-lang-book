fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // remove s wont affect word

    println!("word: {}", word);

    // string slice
    let g = String::from("hello world");
    let hello = &g[0..5];  // 0-5 char, look like python slice
    let world = &g[6..11];

    println!("hello: {}, world: {}", hello, world);

    // slice until end of string
    let s = String::from("hello world");
    let len = s.len();
    let slice1 = &s[3..len];
    let slice2 = &s[6..];

    println!("slice1: {}, slice2: {}", slice1, slice2);

    // first_world will be same memory as s
    let first_world = first_word2(&s);

    // if clear, first_world will be empty string
    // s.clear();

    println!("first_world: {}", first_world);

    // String is full object
    // str is part of String
}

// usize unsigned integer type
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // loop each bytes output number and element using enumerate
    for (i, &item) in bytes.iter().enumerate() {
        // b' ' space string
        if item == b' ' {
            return i;
        }
    }

    // return if empty string
    s.len()
}

fn first_word2(s: &String) -> &str { // return borrow partial string
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]  // return slice 0 to i
        }
    }

    &s[..] // return all
}
