mod shape;
mod shapes;

use shape::Shape;
use shapes::{Circle, Square};

// TODO: Implement the total_area function
// It should accept a Vec<Box<dyn Shape>> and return the sum of all areas as f64
fn total_area(shapes: Vec<Box<dyn Shape>>) -> f64 {
    // TODO: Calculate and return the sum of all shape areas
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let radius: f64 = input1.trim().parse().expect("Invalid number");

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let side: f64 = input2.trim().parse().expect("Invalid number");

    // TODO: Create a Circle with the given radius
    let mycircle = Circle {radius: radius };

    // TODO: Create a Square with the given side
    let mysquare = Square { side: side };

    // TODO: Store both shapes in a Vec<Box<dyn Shape>>
    let myshapes: Box<dyn Shape> = Box::new(mycircle);
    let myshapes2: Box<dyn Shape> = Box::new(mysquare);
    let shapes_vec: Vec<Box<dyn Shape>> = vec![myshapes, myshapes2];

    // TODO: Call total_area and print the result in the format: "Total area: {total}"
    let total = total_area(shapes_vec);
    println!("Total area: {}", total);  
}
