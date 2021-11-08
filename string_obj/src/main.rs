fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let data_str = data.to_string();
    let data_str2 = "initial contents".to_string();

    dbg!(s, data, data_str, data_str2);

    let s1 = String::from("hello world");
    let hello = String::from("你好");
    dbg!(s1, hello);

    let mut s2 = String::from("foo");
    s2.push_str("bar");
    let s2a = "bar";
    s2.push_str(s2a); // push a str
    dbg!(&s2);

    s2.push('.'); // push a char
    dbg!(&s2);

    // join string with
    let hello = String::from("hello");
    let world = String::from("world");
    // hello borrow, world reference
    let hello_world = hello + " " + &world;
    dbg!(hello_world);

    // join string simple
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let tic_tac_toe = tic + "-" + &tac + "-" + &toe;
    dbg!(tic_tac_toe);

    // new method: format!
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let ttt = format!("{}-{}-{}", tic, tac, toe); // borrow all
    dbg!(ttt);

    // important: string cannot index [0]
    // let s1 = String::from("hello");
    // let h = s1[0];

    // important: string cannot slice using 0..5
    // let hello = "hello";
    // let s = &hello[0];
    // dbg!(s);

    // string loop
    for c in "hello".chars() {
        println!("{}", c);
    }

    // println!("Hello, world!");
}
