mod profile;

use profile::UserProfile;

fn main() {
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let username = input1.trim().to_string();

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let age: u32 = input2.trim().parse().expect("Failed to parse age");

    // TODO: Create a UserProfile using the constructor

    let my_user = UserProfile::new(username, age);

    // TODO: Use the getter methods to print the username and age
    // Format:
    // Username: {username}
    // Age: {age}

    println!("Username: {}", my_user.username());
    println!("Age: {}", my_user.age());

}
