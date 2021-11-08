// like typescript define type of User
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    // non mutable
    let user1 = User {
        active: true,
        username: String::from("someone"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    // unable to change because struct are non mutable
    // user1.email = String::from("another@example.com");

    println!("Hello, {}!", user1.username);

    let user2 = build_user(
        String::from("somepeople@example.com"),
        String::from("somepeople")
    );

    println!("user2: {}", user2.username);

    let user3 = User {
        active: user1.active,
        sign_in_count: user1.sign_in_count,
        email: String::from("new@example.com"),
        username: user1.username
    };

    println!("username3: {}", user3.username);

    // cannot use user1 because value inside already borrowed
    let user4 = User {
        email: String::from("user4@example.com"),
        ..user3  // same as javascript
    };
    println!("username4: {}", user4.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // println!("black: {}, origin: {}", black, origin);

    // nothing struct
    let subject = AlwaysEqual;

}

fn build_user(email: String, username: String) -> User {
    User {
        email,  // same as javascript object
        username,
        active: true,
        sign_in_count: 1
    }
}