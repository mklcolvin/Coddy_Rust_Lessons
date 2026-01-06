mod locker;

use locker::Locker;

fn main() {
    // Read inputs
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let initial_code: u32 = input.trim().parse().expect("Invalid number");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let contents = input.trim().to_string();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let first_attempt: u32 = input.trim().parse().expect("Invalid number");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let new_code: u32 = input.trim().parse().expect("Invalid number");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let second_attempt: u32 = input.trim().parse().expect("Invalid number");

    // TODO: Create a new Locker with initial_code and contents

    let mut my_locker = Locker::new(initial_code, contents);

    // TODO: Attempt to get contents with first_attempt code

    let result = my_locker.get_contents(first_attempt);

    // Print: "Attempt with {code}: {result}"

    println!("Attempt with {}: {}", first_attempt, result);

    // TODO: Change the locker code to new_code

    let _result = my_locker.set_code(new_code);

    // Print: "Code changed"

    println!("Code changed");

    // TODO: Attempt to get contents with second_attempt code

    let result = my_locker.get_contents(second_attempt);

    // Print: "Attempt with {code}: {result}"

    println!("Attempt with {}: {}", second_attempt, result);

}
