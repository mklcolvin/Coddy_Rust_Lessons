// TODO: Define a public trait called Summarizable
// The trait should have a single method signature:
// - summarize(&self) -> String
// Remember: In a trait definition, you only declare the method signature
// with a semicolon, no body!

pub trait Summarizable {
    fn summrize(&self) -> String;
}