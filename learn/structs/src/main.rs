use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.width * self.height
    }

    fn perimeter(&self) -> u64 {
        2 * (self.height + self.width)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main() {
    let mut input_width = String::new();
    let mut input_height = String::new();

    println!("Enter width of rectangle:");
    io::stdin().read_line(&mut input_width)
        .expect("Input error");
    println!("Enter height of rectangle:");
    io::stdin().read_line(&mut input_height)
        .expect("Input error");

    let rectangle = Rectangle {
        width: match input_width.trim().parse() {
            Ok(input) => input,
            Err(_) => {
                panic!("Expected a number");
            }
        },
        height: match input_height.trim().parse() {
            Ok(input) => input,
            Err(_) => {
                panic!("Expected a number");
            }
        },
    };

    println!("Our Rectangle is {:#?}", rectangle);
    println!("Area of rectangle is {}", rectangle.area());
    println!("Perimeter of rectangle is {}", rectangle.perimeter());
}