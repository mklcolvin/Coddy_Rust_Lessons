use std::io;

// TODO: Define the Circle struct here

struct Circle {
    radius: f64,
}

// TODO: Add the implementation block for Circle with circumference and diameter methods

impl Circle {
    fn circumference(&self)-> f64 {
        2.0 * 3.14159 * self.radius
    }

    fn diameter(&self) -> f64 {
        2.0 * self.radius
    }
}
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let radius: f64 = input.trim().parse().expect("Invalid number");
    
    // TODO: Create a Circle instance and print the circumference and diameter
    
    let my_circle: Circle = Circle {
        radius: radius,
    };

    println!("{}", my_circle.circumference());
    println!("{}", my_circle.diameter());
}