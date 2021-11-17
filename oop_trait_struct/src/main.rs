use oop_trait_struct::{ Screen, Button, SelectBox };

fn main() {
    // let button = Button {
    //     width: 100,
    //     height: 100,
    //     label: String::from("Click me!"),
    // };
    //
    // button.draw();

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ]
    };

    screen.run();
}






