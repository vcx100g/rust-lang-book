use std::thread;
use std::time::Duration;
use closure_short_func::Cacher;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

// code clean format with only run once expensive function
fn generate_workout2(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);

    // closure function return value "num"
    // only run when call expensive_closure
    // like () => {} in js
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}


fn generate_workout3(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);

    // using struct with custom closure function
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    // generate_workout1(simulated_user_specified_value, simulated_random_number);
    // generate_workout2(simulated_user_specified_value, simulated_random_number);
    generate_workout3(simulated_user_specified_value, simulated_random_number);

    // function
    // fn add_one_v1 (x: u32) -> u32 { x + 1 };

    // other type of closure function
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| { x + 1 };
    // let add_one_v4 = |x| x + 1; // shortest

    // diff data type wont support
    // let example_closure = |x| x;
    // first string second integer
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);

    let x = 4;
    // closure function use out scope variables
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y)); // true

    // if using function will not work for x
    // fn equal_to_x2(z: i32) -> bool {
    //     z == x
    // }

    // let x1 = vec![1, 2, 3];
    // // new keyword: move concurrency
    // // concurrency cannot share variables x
    // let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);
    //
    // let y = vec![1, 2, 3];
    // assert!(equal_to_x(y));
}
