// use gui::Draw;

// draw trait
pub trait Draw {
    fn draw(&self);
}

pub trait Clone {
    fn clone(&self) -> Self;
}

// screen trait
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// screen use component
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// use dynamic type must draw; not work duck typing
// pub struct Screen2<T: Draw> {
//     pub components: Vec<T>,
// }
//
// impl<T> Screen2<T> where T: Draw
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("Button {:?}", self);
    }
}



#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("SelectBox {:?}", self);
    }
}

