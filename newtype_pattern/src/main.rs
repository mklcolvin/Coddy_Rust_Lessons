mod user_types;

use user_types::{Username, UserId};

// TODO: Create a function called `display_user` that takes:
// - First parameter: UserId
// - Second parameter: Username
// The function should print: "User #{id}: {username}"
// Use the .value() method to access the inner values

fn display_user(id: UserId, username: Username) {
    println!("User #{}: {}", id.value(), username.value())
}

fn main() {
    // Read input
    let mut id_input = String::new();
    std::io::stdin().read_line(&mut id_input).expect("Failed to read line");
    let id: u32 = id_input.trim().parse().expect("Invalid number");
    
    let mut username_input = String::new();
    std::io::stdin().read_line(&mut username_input).expect("Failed to read line");
    let username = username_input.trim().to_string();
    
    // TODO: Create instances of UserId and Username using the inputs
    let user_id = UserId(id);
    let user_name = Username(username);

    
    // TODO: Call display_user with the correct arguments
    display_user(user_id, user_name);
    
}
