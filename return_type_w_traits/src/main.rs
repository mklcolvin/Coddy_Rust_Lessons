mod greetings;

use greetings::Greet;

fn main() {
    // Read the greeting message from input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let message = input.trim().to_string();

    // TODO: Call create_greeting with the message
    // Note: You don't know the concrete type - only that it implements Greet!
    
    let greeting = greetings::create_greeting(message);

    // TODO: Call the greet method and print the result
    println!("{}", greeting.greet());
    
}
