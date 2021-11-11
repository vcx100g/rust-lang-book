// closure in struct
pub struct Cacher<T> where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    // new return another cacher with close function
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                // run closure function
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
