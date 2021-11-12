// all iterator look like this
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // other method implementation
}

// iter is faster than for loop
fn main() {
    let v1 = vec![1, 2, 3, 4, 5];

    // make this an iterator, use next() to get each item
    let v1_iter = v1.iter();
    println!("{:?}", v1_iter);
}
