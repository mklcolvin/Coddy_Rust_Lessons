use std::io;

// TODO: Define the Rectangle struct here

struct Rectangle {
    width: u32,
    height: u32,
}


// TODO: Add the implementation block for Rectangle with:
// - new: associated function that takes width and height, returns Rectangle
// - area: method that takes &self, returns u32
// - scale: method that takes &mut self and factor, multiplies width and height

impl Rectangle {

    fn new(new_width: u32, new_height: u32) -> Rectangle {
        Rectangle {
            width: new_width,
            height: new_height
        }

    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn scale(&mut self, scale_factor: u32) {
        self.width = self.width * scale_factor;
        self.height = self.height * scale_factor
    }
}

fn main() {
    let mut input = String::new();
    
    // Read width
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let width: u32 = input.trim().parse().expect("Invalid number");
    
    input.clear();
    
    // Read height
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let height: u32 = input.trim().parse().expect("Invalid number");
    
    input.clear();
    
    // Read scale factor
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let factor: u32 = input.trim().parse().expect("Invalid number");
    
    // TODO: Create a mutable Rectangle using Rectangle::new

    let mut my_rectangle = Rectangle::new(width, height);
    
    // TODO: Print the initial area

    println!("{}", my_rectangle.area());
    
    // TODO: Scale the rectangle by the factor

    my_rectangle.scale(factor);
    
    // TODO: Print the new area

    println!("{}", my_rectangle.area());

}