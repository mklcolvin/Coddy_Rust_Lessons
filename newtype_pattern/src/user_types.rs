// TODO: Define a public newtype struct called `Username` that wraps a String
// pub struct Username(...)

pub struct Username(pub String);

// TODO: Define a public newtype struct called `UserId` that wraps a u32
// pub struct UserId(...)

pub struct UserId(pub u32);

// TODO: Implement a method called `value` for Username
// that returns a reference to the inner String (&String)
impl Username {
    pub fn value(&self) -> &String {
        &self.0
    }
}

// TODO: Implement a method called `value` for UserId
// that returns a reference to the inner u32 (&u32)
impl UserId {
    pub fn value(&self) -> &u32{
        &self.0
    }
}
