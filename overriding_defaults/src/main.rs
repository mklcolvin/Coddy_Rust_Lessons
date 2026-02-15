mod notifiable;
mod alerts;

use alerts::{GenericAlert, UrgentAlert};
use notifiable::Notifiable;

fn main() {
    // Read input for the urgent alert message
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let message = input.trim().to_string();

    // TODO: Create a GenericAlert instance

     let genetic = GenericAlert;
    // TODO: Create an UrgentAlert instance with the input message
    let urgent = UrgentAlert { message };

    // TODO: Print the notification from GenericAlert
    println!("{}", genetic.notification());

    // TODO: Print the notification from UrgentAlert
    println!("{}", urgent.notification());  
}
