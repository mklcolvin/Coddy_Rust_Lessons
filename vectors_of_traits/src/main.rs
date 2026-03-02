mod notifications;

use notifications::{Notify, Email, Sms, Push};

fn main() {
    // Read inputs
    let mut recipient = String::new();
    std::io::stdin().read_line(&mut recipient).expect("Failed to read line");
    let recipient = recipient.trim().to_string();

    let mut phone = String::new();
    std::io::stdin().read_line(&mut phone).expect("Failed to read line");
    let phone = phone.trim().to_string();

    let mut device = String::new();
    std::io::stdin().read_line(&mut device).expect("Failed to read line");
    let device = device.trim().to_string();

    // TODO: Create a Vec<Box<dyn Notify>> to hold different notification types
    // TODO: Create Email, Sms, and Push instances using the inputs
    // TODO: Add them to the vector in order (Email, Sms, Push) using Box::new()
    // TODO: Print the result of calling send() on the first element

    let mut notifications: Vec<Box<dyn Notify>> = Vec::new();
    notifications.push(Box::new(Email { recipient }));
    notifications.push(Box::new(Sms { phone }));
    notifications.push(Box::new(Push { device }));
    println!("{}", notifications[0].send());
    
}
