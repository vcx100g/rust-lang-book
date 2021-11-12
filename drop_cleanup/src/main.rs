struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // this function will run when out scope cleanup
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff")
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };

    println!("CustomSmartPointer created.");

    // call drop before out of scope cleanup
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
