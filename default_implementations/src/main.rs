mod notifiable;
mod alerts;

use notifiable::Notifiable;
use alerts::{EmailAlert, SystemAlert};

fn main() {
    // TODO: Create an instance of EmailAlert

    let email = EmailAlert;
    
    // TODO: Create an instance of SystemAlert

    let system = SystemAlert;
    
    // TODO: Print the notification from EmailAlert
    
    println!("{}", email.notify());

    // TODO: Print the notification from SystemAlert

    println!("{}", system.notify());

}
