use crate::notifiable::Notifiable;

// TODO: Define a public unit struct GenericAlert
// It should implement Notifiable with an empty impl block (use default)

// TODO: Define a public struct UrgentAlert with a public message field (String)
// It should implement Notifiable and override the notify method
// to return "URGENT: {message}" where {message} is the stored message

pub struct GenericAlert;

impl Notifiable for GenericAlert {

}

pub struct UrgentAlert {
    pub message: String,
}

impl Notifiable for UrgentAlert {
    fn notification(&self) -> String {
        format!("URGENT: {}", self.message)
    }
}   