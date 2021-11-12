
#[test]
fn iterator_demo() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // "sum" builtin function in iter
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    // "map" builtin function in iter
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4])


}

// using iterator with struct
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

fn shoe_in_size(shoe: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into iter change vector into iter
    // filter iter builtin function
    shoe.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod shoe_tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoe_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        )
    }
}


struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// create custom iterator for struct
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // everytime it call next, add 1
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_method() {
    // other iterator builtin function
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}