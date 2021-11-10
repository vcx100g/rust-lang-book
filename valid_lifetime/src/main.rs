/*
lifetime syntax is about connecting the lifetimes of
 various parameters and return values of functions
memory-safe operations and disallow operations that
 would create dangling pointers or otherwise violate memory safety

Lifetimes on function or method parameters are called input lifetimes,
 and lifetimes on return values are called output lifetimes

Lifetime is use to solve borrow out of scope problem in rust
 */

// new keyword: 'a or 'static 'lifetime
// lifetime keyword

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

use std::fmt::Display;

// with this x and y have same life time in function
// both x and y will stay until each last return called
// usually use 'a but using 'lifetime more easy understand
fn longest2<'lifetime>(x: &'lifetime str, y: &'lifetime str) -> &'lifetime str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// short version using 'a
fn longest3<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// error because result not longer live outside function scope
// fn longest4<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// lifetime for struct, part property live until struct end
struct ImportantExcerpt<'a> {
    part: &'a str
}

// lifetime elision > if not lifetime set, all auto set as same lifetime
// when 1 function 1 parameters 1 result
// this function s and return have same lifetime
fn first_word(s: &str) -> &str {
// fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

// multi parameters dont have same lifetime
// so return type not sure is a lifetime or b lifetime
// fn longest_ab<'a, 'b>(x: &'a str, y: &'b str) -> &str {}

// impl have same lifetime as struct
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// lifetime in trait interface function
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where T: Display {
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // borrow scope check
    // let r;
    // {
    //     let x= 5;
    //     r = &x; // not longer exists when out of scope
    // }
    // println!("r: {}", r); // so this will not run cause error

    let string1 = String::from("abcd");
    let string2 = "xyz";

    // this cannot be borrow, either x ro y is out scope at the end
    // mean x return by function, then y is not longer live
    // or y return by function, then x is not longer live
    // let result = longest(string1.as_str(), string2);
    let result = longest2(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    // using inner scope for lifetime
    {
        let string3 = String::from("xyz");
        let result2 = longest2(string1.as_str(), string3.as_str());
        println!("The longest string is: {}", result2);
    }
    // error because string3 is out scope at the end; not longer live
    // println!("The longest string is: {}", result2);

    // longest4(string1.as_str(), string2);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
    println!("part: {}", i.part);
    println!("first word: {}", first_word(&novel));

    // new keyword: 'static live entire program
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);

    i.announce_and_return_part("here");

    let a = "some";
    let b = "none";
    let result = longest_with_an_announcement(a, b, "Something");
    println!("result: {}", result);
}
