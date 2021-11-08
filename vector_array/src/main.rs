// vector are like array but implement in rust
fn main() {
    let v: Vec<i32> = Vec::new();
    dbg!(v);

    let arr = [1, 2, 3];

    // new method: vec! craete vector
    let v1 = vec![1, 2, 3];
    dbg!(v1, arr);

    // get element in vector
    let v2 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v2[2];

    // v2.get is same as above but easy
    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }

    // not borrow vector
    let mut v3 = vec![1, 2, 3, 4, 5];
    let first = &v3[0]; // already borrow to first
    // v3.push(6); // error: cannot update here
    println!("The first element is: {}", first);

    // for loop vector
    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{}", i);
    }

    // update vector
    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        // dereference operator (*) to get to the value in i
        *i += 60;
        println!("i: {}", i)
    }

    // enum vector
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue"))
    ];
    for i in &row {
        println!("{:?}", i);
    }


    // println!("Hello, world!");
}
