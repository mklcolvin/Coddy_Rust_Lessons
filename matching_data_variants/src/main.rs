mod command;

use command::Command;

fn main() {
    // Read inputs
    let mut say_message = String::new();
    std::io::stdin().read_line(&mut say_message).expect("Failed to read line");
    let say_message = say_message.trim().to_string();

    let mut direction = String::new();
    std::io::stdin().read_line(&mut direction).expect("Failed to read line");
    let direction = direction.trim().to_string();

    let mut steps_input = String::new();
    std::io::stdin().read_line(&mut steps_input).expect("Failed to read line");
    let steps: u32 = steps_input.trim().parse().expect("Failed to parse steps");

    let mut num1_input = String::new();
    std::io::stdin().read_line(&mut num1_input).expect("Failed to read line");
    let num1: i32 = num1_input.trim().parse().expect("Failed to parse num1");

    let mut num2_input = String::new();
    std::io::stdin().read_line(&mut num2_input).expect("Failed to read line");
    let num2: i32 = num2_input.trim().parse().expect("Failed to parse num2");

    // TODO: Create a Say command using say_message and execute it
    let speak_message = Command::Say(say_message);
    println!("{}", speak_message.execute());


    // TODO: Create a Move command using direction and steps, and execute it
    let move_message = Command::Move { direction: direction, steps: steps };
    println!("{}", move_message.execute());

    // TODO: Create a Calculate command using num1 and num2, and execute it
    let calc_message = Command::Calculate(num1, num2);
    println!("{}", calc_message.execute());

    // TODO: Print the results of each command execution
}
