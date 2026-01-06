// TODO: Define a public UserProfile struct with private fields:
// - username: String
// - age: u32

pub struct UserProfile {
    username: String,
    age: u32,
}

// TODO: Implement the following for UserProfile:
// - pub fn new(username: String, age: u32) -> Self
// - pub fn username(&self) -> &String (getter for username)
// - pub fn age(&self) -> u32 (getter for age)

impl UserProfile {

    pub fn new(username: String, age: u32) -> Self {
        UserProfile { username: username, age: age }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn age(&self) -> u32 {
        self.age
    }
}