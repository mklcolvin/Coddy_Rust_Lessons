// TODO: Derive the Debug trait so the enum can be printed with {:?}
#[derive(Debug)]
// TODO: Define a public Notification enum with three variants:
pub enum Notification {
    Alert(String),
    Reminder {title: String, minutes: u32},
    Dismiss,
}
// - Alert: holds a single String message (tuple syntax)
// - Reminder: holds named fields: title (String) and minutes (u32)
// - Dismiss: holds no data
