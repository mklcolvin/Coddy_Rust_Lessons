// TODO: Define a public generic struct SmartBox<T> with a single field 'value' of type T
pub struct SmartBox<T> {
    value: T,
}

// TODO: Implement SmartBox<T> with:
// - A 'new' associated function that:
//   - Takes a value of type T
//   - Prints "SmartBox created"
//   - Returns a new SmartBox containing the value
//
// - A 'get' method that:
//   - Returns a reference to the inner value (&T)
impl<T> SmartBox<T> {
    pub fn new(value: T) -> Self {
        println!("SmartBox created");
        SmartBox { value }
    }

    pub fn get(&self) -> &T {
        &self.value
    }
}

// TODO: Implement the Drop trait for SmartBox<T>
// - The drop method should print "SmartBox dropped"
impl<T> Drop for SmartBox<T> {
    fn drop(&mut self) {
        println!("SmartBox dropped");
    }
}
