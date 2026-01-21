use crate::notifiable::Notifiable;

// TODO: Define a public unit struct called EmailAlert

pub struct EmailAlert;

// TODO: Define a public unit struct called SystemAlert

pub struct SystemAlert;

// TODO: Implement Notifiable for EmailAlert with an empty impl block
// (This will use the default implementation)

impl Notifiable for EmailAlert {}

// TODO: Implement Notifiable for SystemAlert with an empty impl block
// (This will use the default implementation)

impl Notifiable for SystemAlert {}

