mod coordinates;

use coordinates::{GridPoint, NamedLocation};

fn main() {
    // Read inputs
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let x: i32 = input1.trim().parse().expect("Invalid number");

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let y: i32 = input2.trim().parse().expect("Invalid number");

    let mut input3 = String::new();
    std::io::stdin().read_line(&mut input3).expect("Failed to read line");
    let name = input3.trim().to_string();

    // TODO: Create a GridPoint instance with x and y

    let mygrid = GridPoint { x: x, y: y };

    // TODO: Demonstrate implicit copy by assigning to another variable
    // (GridPoint implements Copy, so this creates a copy automatically)

    let newgrid = mygrid;

    // TODO: Print original and copied point
    // Format: "Original point: ({x}, {y})"
    // Format: "Copied point: ({x}, {y})"

    println!("Original point: ({}, {})", mygrid.x, mygrid.y);
    println!("Copied point: ({}, {})", newgrid.x, newgrid.y);
    
    // TODO: Create a NamedLocation instance with name, x, and y

    // TODO: Demonstrate explicit clone (NamedLocation only implements Clone, not Copy)
    // Use .clone() to create a duplicate

    // TODO: Print original and cloned location
    // Format: "Original location: {name} at ({x}, {y})"
    // Format: "Cloned location: {name} at ({x}, {y})"
}
