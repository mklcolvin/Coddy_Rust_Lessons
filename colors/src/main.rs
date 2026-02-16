mod color;

use color::Color;

fn main() {
    // Read the six RGB values
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let r1: u8 = input.trim().parse().expect("Invalid input");

    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let g1: u8 = input.trim().parse().expect("Invalid input");

    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let b1: u8 = input.trim().parse().expect("Invalid input");

    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let r2: u8 = input.trim().parse().expect("Invalid input");

    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let g2: u8 = input.trim().parse().expect("Invalid input");

    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let b2: u8 = input.trim().parse().expect("Invalid input");

    // TODO: Create two Color instances using the parsed values

    let first_color = Color { red: r1, green: g1, blue: b1 };
    let second_color = Color { red: r2, green: g2, blue: b2 };

    // TODO: Print Color 1 in the format: Color 1: rgb(r, g, b)

    println!("Color 1: rgb({}, {}, {})", first_color.red, first_color.green, first_color.blue);

    // TODO: Print Color 2 in the format: Color 2: rgb(r, g, b)

    println!("Color 2: rgb({}, {}, {})", second_color.red, second_color.green, second_color.blue);

    // TODO: Compare the two colors using == and print: Colors match: {true/false}

    println!("Colors match: {}", first_color == second_color);

}
