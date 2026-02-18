mod point;

use point::Point;

fn main() {
    // Read four inputs: x1, y1, x2, y2
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let x1: i32 = input.trim().parse().expect("Invalid input");

    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let y1: i32 = input.trim().parse().expect("Invalid input");

    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let x2: i32 = input.trim().parse().expect("Invalid input");

    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let y2: i32 = input.trim().parse().expect("Invalid input");

    // TODO: Create two Point instances using the inputs

    let first_point = Point { x: x1, y: y1 };
    let second_point = Point { x: x2, y: y2 };

    // TODO: Print the first point using debug format {:?}
    // Format: "Debug: Point { x: ..., y: ... }"

    println!("Debug: {:?}", first_point);

    // TODO: Print the second point using display format {}
    // Format: "Display: (..., ...)"

    println!("Display: ({})", second_point); 
    // TODO: Compare the two points and print the result
    // Format: "Equal: true" or "Equal: false"
    let are_equal = first_point == second_point;
    println!("Equal: {}", are_equal);

}
