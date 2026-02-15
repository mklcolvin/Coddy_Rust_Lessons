// TODO: Define a public Notifiable trait
// The trait should have a notify(&self) -> String method
// with a default implementation that returns "Alert: Something happened!"
pub trait Notifiable {
    fn notification(&self) -> String {
        String::from("Alert: Something happened!")
    }
}
