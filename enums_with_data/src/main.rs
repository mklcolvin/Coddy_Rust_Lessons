mod notification;

use notification::Notification;

fn main() {
    // Read inputs
    let mut alert_message = String::new();
    std::io::stdin().read_line(&mut alert_message).expect("Failed to read line");
    let alert_message = alert_message.trim().to_string();

    let mut reminder_title = String::new();
    std::io::stdin().read_line(&mut reminder_title).expect("Failed to read line");
    let reminder_title = reminder_title.trim().to_string();

    let mut minutes_input = String::new();
    std::io::stdin().read_line(&mut minutes_input).expect("Failed to read line");
    let reminder_minutes: u32 = minutes_input.trim().parse().expect("Failed to parse minutes");

    // TODO: Create an Alert notification using the alert_message
    let my_alert = Notification::Alert(alert_message);

    // TODO: Create a Reminder notification using reminder_title and reminder_minutes
    let my_reminder = Notification::Reminder {title: reminder_title, minutes: reminder_minutes };

    // TODO: Create a Dismiss notification

    let my_dismiss_notification = Notification::Dismiss;

    // TODO: Print each notification using debug formatting {:?}
    // Each notification should be printed on its own line

    println!("{:?}", my_alert);

    println!("{:?}", my_reminder);

    println!("{:?}", my_dismiss_notification);

}
