// TODO: Define a public generic Point<T> struct with public fields x and y


// TODO: Implement methods for Point<T>
impl<T> Point<T> {
    // TODO: Implement the new associated function
    // pub fn new(...) -> Self { ... }
    pub fn new(x, y) -> Self {

    }
}

// TODO: Implement the translate method
// This requires T to support addition and copying
// Use the bound: T: std::ops::Add<Output = T> + Copy
impl<T: std::ops::Add<Output = T> + Copy> Point<T> {
    // TODO: Implement translate method that returns a new Point
    // pub fn translate(...) -> Self { ... }
}
