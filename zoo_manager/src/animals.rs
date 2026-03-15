// TODO: Define a public Speak trait with a speak method
// The speak method should take &self and return a String

pub trait Speak {
    fn speak(&self) -> String;
}

// TODO: Create a public Dog struct with a public name field (String)

pub struct Dog {
    pub name: String,
}

// TODO: Create a public Cat struct with a public name field (String)

pub struct Cat {
    pub name: String,
}

// TODO: Create a public Bird struct with a public name field (String)

pub struct Bird {
    pub name: String,
}

// TODO: Implement the Speak trait for Dog
// Should return "{name} says: Woof!"

impl Speak for Dog {
    fn speak(&self) -> String {
        format!("{} says: Woof!", self.name)
    }
}

// TODO: Implement the Speak trait for Cat
// Should return "{name} says: Meow!"

impl Speak for Cat {
    fn speak(&self) -> String {
        format!("{} says: Meow!", self.name)
    }
}

// TODO: Implement the Speak trait for Bird
// Should return "{name} says: Tweet!"

impl Speak for Bird {
    fn speak(&self) -> String {
        format!("{} says: Tweet!", self.name)
    }
}