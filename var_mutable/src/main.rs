fn main() {
    // mutable var
    let mut x:i32 = 5;
    println!("x original: {}", x);

    x = 6;
    println!("x mutable: {}", x);

    // constants must uppercase
    const THREE_HOURS_IN_SECONDS: u32 = 60 *60*3;
    println!("3 hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    let y = 7;
    let y = y + 1;
    {
        let y = y * 2;
        println!("y inside bracket: {}", y);
    }

    println!("y outside bracket: {}", y);

    // cannot use empty string as variable
    // let spaces:String = "   ";
    // let spaces = spaces.len();
}
