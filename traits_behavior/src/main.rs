// traits are interface in typescript
// can use as implement structure
// also can use as parameter type
// and return type
use std::fmt::Display;
use std::fmt::Debug;
use traits_behavior::Summary2;

// define interface type trait
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

// use trait for struct method
impl Summary2 for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.username, self.content)
    }
}

pub struct Tweet2 {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary2 for Tweet2 {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// use trait in parameters
pub fn notify(item: &impl Summary2) {
    println!("Notify breaking news! {}", item.summarize());
}

// use trait with generic type for parameters
pub fn notify2<T: Summary2>(item: &T) {
    println!("Notify headline! {}", item.summarize());
}

pub fn notify3(item1: &impl Summary, item2: &impl Summary2) {}
pub fn notify4<T: Summary2>(item: &T, item2: &T) {}

// use trait with generic type extend
pub fn notify5(item: &(impl Summary2 + Display)) {}
pub fn notify5a<T: Summary2 + Display>(item: &T) {}

// 2 generic type with trait
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

// same as above use 2 generic type trait with where
// new keyword: where
fn some_function2<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
        U: Clone + Debug
{
    0
}

// return a impl trait
fn returns_summarizable() -> impl Summary2 {
    Tweet2 {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// double generic type for trait not work; cause error
// fn returns_summarizable2(switch: bool) -> impl Summary2 {
//     match switch {
//         true => NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         },
//         false => Tweet2 {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// now largest can work for both integer and char
// PartialOrd: any value can compare by order, number or char
// Copy: value that can duplicate by bits
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic type for impl struct type and parameters
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let news_article = NewsArticle {
        headline: String::from("Bitcoin Soars Past $68K"),
        location: String::from("United States"),
        author: String::from("Muyao Shen"),
        content: String::from("The total market capitalization of all cryptocurrencies neared a milestone of $3 trillion.")
    };
    println!("1 news headline: {}", news_article.summarize());

    let tweet = Tweet {
        username: String::from("horse book"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // using lib
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    println!("Author: {}", article.summarize_author());

    let tweet2 = Tweet2 {
        username: String::from("horse book"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("New tweet: {}", tweet2.summarize());
    println!("Username: {}", tweet2.summarize_author());

    notify(&tweet2);
    notify2(&article);

    let tweet3 = returns_summarizable();
    println!("New tweet: {}", tweet3.summarize());

    // largest
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // pair declare with type
    let pair = Pair::new(10, 30);
    pair.cmp_display();
}
